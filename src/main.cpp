#include "connection/i3client.hpp"
#include "converters.hpp"
#include "direction"
#include "grid.hpp"
#include "tabs.hpp"
#include "utils/call.hpp"
#include "utils/logging.hpp"
#include "utils/stringf.hpp"

#include <docopt/docopt.h>
#include <nlohmann/json.hpp>

#include <algorithm>
#include <climits>
#include <cstdio>
#include <iostream>
#include <sstream>
#include <string>
#include <thread>
#include <vector>
#include <optional>

#include <chrono>
#include <thread>

char USAGE[] = R"(i3 geometric window switcher

Usage:
  i3switch (next | prev) [wrap]                 # order movement (tabs, stacks, windows)
  i3switch number <num>                         # order movement (tabs, stacks, windows)
  i3switch (left | up | right | down) [wrap]    # direction movement (grid)
  i3switch (-h | --help)                        # show this help
)";

std::map<std::string, Direction1d> direction_1d_map{
    {"prev", Direction1d::PREV},
    {"next", Direction1d::NEXT}};

std::map<std::string, Direction2d> direction_2d_map{
    {"left", Direction2d::LEFT},
    {"right", Direction2d::RIGHT},
    {"up", Direction2d::UP},
    {"down", Direction2d::DOWN}};

logging::Logger logger;

int main(int argc, char *argv[]) {
    using nlohmann::json;

    logger.configure("%s:%s()  ", __FILENAME__, __func__);

    size_t order = 0;
    bool wrap = false;
    std::optional<Direction2d> direction_2d;
    std::optional<Direction1d> direction_1d;

    {
        auto args = docopt(argv + 1, argv + argc);

        // Verify args
        if (args["number"])
            order = atoi(args["<num>"]);
            if (order == 0) {
                std::cerr << "Tab order must be greater than 0";
                return 1;
            }
        }

        for (auto pair : direction_2d_map) {
            if (args[pair.first]) {
                direction_2d = pair.second;
                break;
            }
        }

        for (auto pair : direction_1d_map) {
            if (args[pair.first]) {
                direction_1d = pair.second;
                break;
            }
        }

        wrap = args["wrap"];
    }

    // Get socket directory name
    std::string i3_socket_path = call("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");
    json root = json::parse(result);

    std::string target_id;
    if (order > 0 or direction_1d.has_value()) {
        auto tabs = converters::available_tabs(root);
        tabs.dump();
        tabs::Tab const *tab;
        if (order > 0) {
            tab = tabs[order];
        } else {
            tab = tabs.next(direction1d);
            if (tab == nullptr and wrap) {
                tab = tabs.first(direction1d);
            }
        }
        if (tab == nullptr) {
            logger.critical("%s", "Can't switch to tab, tab not found");
        } else {
            target_id = tab->id;
        }
    }
    else if (direction_2d.has_value()) {
        auto visible_nodes = json::visible_nodes(root);

        if (is_floating_focused(visible_nodes)) {
            auto windows = converters::floating(visible_nodes);
        } else {
            auto grid = converters::grid(visible_nodes);
        }
    }
    /// @todo floating windows
    else if (grid_direction_map.count(command) > 0) {
        auto grid = converters::visible_grid(root);
        grid::Window const *window = grid.next(direction1d);
        if (window == nullptr && wrap) {
            window = grid.first(direction1d);
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
