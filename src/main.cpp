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

#include <thread>
#include <chrono>


int main(int argc, char *argv [])
{
    using nlohmann::json;

    logging::Logger log;
    log.configure("%s:%s()  ", __FILENAME__, __func__);

    size_t order;

    enum class Command {help, prev, next, number};
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
            ).doc("tab switching")
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
    //else if (command == Command::left or command == command::up or
    //         command == Command::right or command == Command::down)
    //{
    //    TODO
    //}
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
