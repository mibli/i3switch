#include "connection/i3client.hpp"
#include "converters.hpp"
#include "grid.hpp"
#include "tabs.hpp"
#include "utils/call.hpp"
#include "utils/logging.hpp"
#include "utils/stringf.hpp"

#include <clipp.h>
#include <nlohmann/json.hpp>

#include <algorithm>
#include <climits>
#include <cstdio>
#include <iostream>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

#include <chrono>
#include <thread>

enum class Command {
    help,

    // tabs commands
    prev,
    next,
    number,

    // grid commands
    left,
    up,
    right,
    down
};

std::map<Command, tabs::Direction> tabs_direction_map{
    {Command::prev, tabs::Direction::PREV},
    {Command::next, tabs::Direction::NEXT}};

std::map<Command, grid::Direction> grid_direction_map{
    {Command::left, grid::Direction::LEFT},
    {Command::right, grid::Direction::RIGHT},
    {Command::up, grid::Direction::UP},
    {Command::down, grid::Direction::DOWN}};

logging::Logger logger;

int main(int argc, char *argv[]) {
    using nlohmann::json;

    logger.configure("%s:%s()  ", __FILENAME__, __func__);

    size_t order;

    Command command = Command::help;
    bool wrap = false;

    auto args = (clipp::one_of(
        clipp::option("-h", "--help")
            .set(command, Command::help)
            .doc("show this help message"),
        clipp::one_of(
            clipp::in_sequence(
                clipp::one_of(clipp::required("prev")
                                  .set(command, Command::prev)
                                  .doc("focus previous"),
                              clipp::required("next")
                                  .set(command, Command::next)
                                  .doc("focus next")),
                clipp::option("wrap").set(wrap, true).doc("wrap tabs")),
            clipp::in_sequence(
                clipp::command("number").set(command, Command::number),
                clipp::value("N", order))
                .doc("focus tab by order, where N in [1..]"))
            .doc("tab switching"),
        clipp::in_sequence(
            clipp::option("wrap").set(wrap, true).doc("wrap around edges"),
            clipp::one_of(
                clipp::required("left").set(command, Command::left),
                clipp::required("up").set(command, Command::up),
                clipp::required("right").set(command, Command::right),
                clipp::required("down").set(command, Command::down)))));

    if (not clipp::parse(argc, argv, args) or command == Command::help) {
        std::cout << clipp::make_man_page(args, argv[0]);
        return 0;
    }

    // Verify args
    if (command == Command::number and order == 0) {
        std::cerr << "Tab order must be greater than 0";
    }

    // Get socket directory name
    std::string i3_socket_path = call("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");
    json root = json::parse(result);

    std::string target_id;
    if (command == Command::number or tabs_direction_map.count(command)) {
        auto tabs = converters::available_tabs(root);
        tabs.dump();
        tabs::Tab const *tab;
        if (command == Command::number) {
            tab = tabs[order];
        } else {
            auto direction = tabs_direction_map[command];
            tab = tabs.next(direction);
            if (tab == nullptr and wrap) {
                tab = tabs.first(direction);
            }
        }
        if (tab == nullptr) {
            logger.critical("%s", "Can't switch to tab, tab not found");
        } else {
            target_id = tab->id;
        }
    }
    /// @todo floating windows
    else if (grid_direction_map.count(command)) {
        auto grid = converters::visible_grid(root);
        auto direction = grid_direction_map[command];
        grid::Window const *window = grid.next(direction);
        if (window == nullptr && wrap) {
            window = grid.first(direction);
        }
        if (window == nullptr) {
            logger.warning("%s", "Couldn't find a window to switch to");
        } else {
            target_id = window->id;
            logger.info("id:%s", target_id.c_str());
        }
    }

    if (target_id.empty()) {
        logger.critical("%s", "Failed to find window to switch to");
        return 1;
    }

    std::string request = stringf("[con_id=%s] focus", target_id.c_str());
    logger.info("request: %s", request.c_str());
    auto reply = i3_client.request(i3::RequestType::RUN_COMMAND, request);
    logger.info("response: %s", reply.c_str());

    return 0;
}
