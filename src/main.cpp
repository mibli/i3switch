#include "utils/logging.hpp"
#include "utils/stringf.hpp"
#include "utils/call.hpp"
#include "connection/i3client.hpp"
#include "i3tree.hpp"

#include <nlohmann/json.hpp>
#include <clipp.h>

#include <string>
#include <iostream>
#include <sstream>
#include <cstdio>
#include <thread>
#include <vector>
#include <algorithm>
#include <climits>

#include <thread>
#include <chrono>


int main(int argc, char *argv [])
{
    using nlohmann::json;

    logging::Logger log;
    log.configure("%s:%s()  ", __FILENAME__, __func__);

    size_t order;

    enum class Command {help, prev, next, number, left, up, right, down};
    Command command = Command::help;
    bool wrap = false;

    auto args = (
        clipp::one_of(
            clipp::option("-h", "--help").set(command, Command::help)
                .doc("show this help message"),
            clipp::one_of(
                clipp::in_sequence(
                    clipp::one_of(
                        clipp::required("prev").set(command, Command::prev)
                            .doc("focus previous"),
                        clipp::required("next").set(command, Command::next)
                            .doc("focus next")
                    ),
                    clipp::option("wrap").set(wrap, true)
                        .doc("wrap tabs")
                ),
                clipp::in_sequence(
                    clipp::command("number").set(command, Command::number),
                    clipp::value("N", order)
                ).doc("focus tab by order, where N in [1..]")
            ).doc("tab switching"),
            clipp::one_of(
                clipp::required("left").set(command, Command::left),
                clipp::required("up").set(command, Command::up),
                clipp::required("right").set(command, Command::right),
                clipp::required("down").set(command, Command::down)
            )
        )
    );

    if (not clipp::parse(argc, argv, args) or command == Command::help)
    {
        std::cout << clipp::make_man_page(args, argv[0]);
        return 0;
    }

    // Verify args
    if (command == Command::number and order == 0)
    {
        std::cerr << "Tab order must be greater than 0";
    }

    // Get socket directory name
    std::string i3_socket_path = call("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");
    i3::Tree tree (json::parse(result));

    json current = tree.get_focused_child(tree.root);
    json parent = tree.find_tabbed(current);
    json target;

    if (command == Command::number)
    {
        // switch to tab number
        json nodes = parent["nodes"];
        if (order > nodes.size())
            log.critical("No tab number %d (only %d tabs)", order, nodes.size());

        target = nodes[order - 1];
        target = i3::Tree::get_focused_child(target);
    }
    else if (command == Command::prev or command == Command::next)
    {
        if (command == Command::prev)
            target = i3::Tree::get_delta_child(parent, -1, wrap);
        else
            target = i3::Tree::get_delta_child(parent, +1, wrap);

        if (target == nullptr)
            log.critical("%s", "Can't switch to tab, tab not found");

        target = i3::Tree::get_focused_child(target);
    }
    else if (command == Command::left or command == Command::up or
             command == Command::right or command == Command::down)
    {
        //TODO: clean it up!
        int middle;
        int delta;
        char const *fst_layout;
        char const *snd_layout;
        std::function<bool(json const &, const char *)> matches_layout =
            [](json const &node, const char *layout) {
                return node["layout"] == layout and not node["nodes"].empty();
            };
        std::function<int(json const &)> get_middle;
        std::function<json(json const &)> edgest;

        if (command == Command::left or command == Command::up)
            edgest = [](json const &node) {
                return node["nodes"].back();
            };
        else
            edgest = [](json const &node) {
                return node["nodes"].front();
            };

        if (command == Command::left or command == Command::right)
        {
            get_middle = [](json const &node) {
                json const &rect = node["rect"];
                return rect["y"].get<int>() + (rect["height"].get<int>() / 2);
            };
            middle = get_middle(current);
            delta = command == Command::left ? -1 : +1;
            fst_layout = "splith";
            snd_layout = "splitv";
        }
        else
        {
            get_middle = [](json const &node) {
                json const &rect = node["rect"];
                return rect["x"].get<int>() + (rect["width"].get<int>() / 2);
            };
            middle = get_middle(current);
            delta = command == Command::up ? -1 : +1;
            fst_layout = "splitv";
            snd_layout = "splith";
        }

        std::function<bool(json const &)> can_switch =
            [&matches_layout, &delta, &fst_layout](json const &node) {
                return matches_layout(node, fst_layout) and
                       i3::Tree::get_delta_child(node, delta, false) != nullptr;
            };
        std::function<json(json const &)> middlest =
            [&get_middle, middle](json const &parent) {
                json best = nullptr;
                int best_delta = INT_MAX;
                for (auto const &node : parent["nodes"]) {
                    int node_delta = std::abs(middle - get_middle(node));
                    if (node_delta < best_delta) {
                        best = node;
                        best_delta = node_delta;
                    }
                }
                return best;
            };

        target = tree.find_matching_parent(current, can_switch);
        target = i3::Tree::get_delta_child(target, delta, false);

        while (target != nullptr)
        {
            if (matches_layout(target, fst_layout))
            {
                target = edgest(target);
            }
            else if (matches_layout(target, snd_layout))
            {
                target = middlest(target);
            }
            else if (not target["nodes"].empty())
            {
                target = tree.get_focused_child(target, 1);
            }
            else
                break;
        }
    }
    //else if (left or up or right or down)
    //{
    //    // OLD APPROACH
    //    // lookup slpith layouts until theres one with a node in the
    //    // requested direction in relation to the focused one,
    //    // otherwise wrap on the closest one
    //    //
    //    // NEW APPROACH
    //    // same except instead of taking "focus" into account, try to
    //    // match positions to the cursor.
    //    //
    //    // if (left)
    //    // if (right)
    //    // if (up)
    //    // if (down)
    //}

    uint64_t target_id = target["id"].get<uint64_t>();
    std::string request = stringf("[con_id=%ld] focus", target_id);
    log.info("request: %s", request.c_str());
    auto reply = i3_client.request(i3::RequestType::RUN_COMMAND, request);
    log.info("response: %s", reply.c_str());

    return 0;
}
