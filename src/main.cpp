#include "connection/i3client.hpp"
#include "converters.hpp"
#include "planar.hpp"
#include "linear.hpp"
#include "utils/call.hpp"
#include "utils/logging.hpp"
#include "utils/stringf.hpp"

#include <docopt.h>
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

constexpr char USAGE[] = R"(i3 geometric window switcher

Usage:
  i3switch (next | prev) [wrap]
  i3switch number <num>
  i3switch (left | up | right | down) [wrap]
  i3switch (-h | --help)

Options:
  next          Move focus to next tab/window
  prev          Move focus to previous tab/window
  number <num>  Move focus to tab/window number <num>
  right         Move focus right
  down          Move focus down
  left          Move focus left
  up            Move focus up
  -h --help     Show this help message
)";

std::map<std::string, linear::Direction> direction_1d_map{
    {"prev", linear::Direction::PREV},
    {"next", linear::Direction::NEXT}};

std::map<std::string, planar::Direction> direction_2d_map{
    {"left", planar::Direction::LEFT},
    {"right", planar::Direction::RIGHT},
    {"up", planar::Direction::UP},
    {"down", planar::Direction::DOWN}};

logging::Logger logger;

int main(int argc, char *argv[]) {
    using nlohmann::json;

    logger.configure("%s:%s()  ", __FILENAME__, __func__);

    size_t order = 0;
    bool wrap = false;
    planar::Direction *direction_2d = nullptr;
    linear::Direction *direction_1d = nullptr;

    {
        auto args = docopt::docopt(std::string(USAGE), std::vector<std::string>(argv + 1, argv + argc), true, "0.1.0");

        // Verify args
        if (args["number"].asBool()) {
            order = args["<num>"].asLong();
            if (order == 0) {
                std::cerr << "Tab order must be greater than 0";
                return 1;
            }
        }

        for (auto pair : direction_2d_map) {
            if (args[pair.first].asBool()) {
                direction_2d = &pair.second;
                break;
            }
        }

        for (auto pair : direction_1d_map) {
            if (args[pair.first].asBool()) {
                direction_1d = &pair.second;
                break;
            }
        }

        wrap = args["wrap"].asBool();
    }

    // Get socket directory name
    std::string i3_socket_path = call("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");
    json root = json::parse(result);
    auto visible_nodes = converters::visible_nodes(root);
    auto windows = converters::to_windows(visible_nodes);
    auto floating = converters::floating_windows(windows);
    auto tiled = converters::tiled_windows(windows);

    if (converters::any_focused(floating)) {
        logger.critical("%s", "Floating movement is not yet implemented");
        return 1;
    }

    std::string target_id;
    if (direction_1d or order > 0) {
        auto tabs = converters::available_tabs(root);
        tabs.dump();
        std::string const *tab;
        if (order > 0) {
            tab = tabs[order];
        } else {
            tab = tabs.next(*direction_1d);
            if (tab == nullptr and wrap) {
                tab = tabs.first(*direction_1d);
            }
        }
        if (tab == nullptr) {
            logger.critical("%s", "Can't switch to tab, tab not found");
        } else {
            target_id = *tab;
        }
    }
    else if (direction_2d) {
        auto grid = converters::visible_grid(tiled);
        std::string const *window = grid.next(*direction_2d);
        if (window == nullptr && wrap) {
            window = grid.first(*direction_2d);
        }
        if (window == nullptr) {
            logger.warning("%s", "Couldn't find a window to switch to");
        } else {
            target_id = *window;
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
