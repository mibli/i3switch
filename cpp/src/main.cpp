#include "connection/i3client.hpp"
#include "converters.hpp"
#include "planar.hpp"
#include "linear.hpp"
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
    planar::Direction direction_2d = planar::Direction::INVALID;
    linear::Direction direction_1d = linear::Direction::INVALID;

    {
        auto args = docopt::docopt(std::string(USAGE), std::vector<std::string>(argv + 1, argv + argc), true, "1.1.0");

        // Verify args
        if (args["number"].asBool()) {
            order = args["<num>"].asLong();
            if (order == 0) {
                std::cerr << "Tab order must be greater than 0";
                return 1;
            }
        }

        for (auto &pair : direction_2d_map) {
            if (args[pair.first].asBool()) {
                direction_2d = pair.second;
            }
        }

        for (auto &pair : direction_1d_map) {
            if (args[pair.first].asBool()) {
                direction_1d = pair.second;
                break;
            }
        }

        wrap = args["wrap"].asBool();

        logger.debug("nr: %d, 2d: %d, 1d: %d, wrap: %d", order, static_cast<int>(direction_2d), static_cast<int>(direction_1d), wrap ? 1 : 0);
    }

    // Get socket directory name
    std::string i3_socket_path = call("i3 --get-socketpath");

    // Create socket connection
    i3::Client i3_client(i3_socket_path);

    auto result = i3_client.request(i3::RequestType::GET_TREE, "");
    json root = json::parse(result);
    auto visible_nodes = converters::visible_nodes(root);
    auto windows = converters::to_windows(visible_nodes);
    auto floating = converters::floating(windows);
    auto tiled = converters::tiled(windows);

    std::string target_id;
    if (direction_1d != linear::Direction::INVALID or order > 0) {
        linear::Sequence seq;
        if (converters::any_focused(floating)) {
            seq = converters::as_sequence(floating);
        } else {
            auto nodes = converters::available_tabs(root);
            auto windows = converters::to_windows(nodes);
            seq = converters::as_sequence(windows);
        }
        seq.dump();
        std::string const *window;
        if (order > 0) {
            window = seq[order];
        } else {
            window = seq.next(direction_1d);
            if (window == nullptr and wrap) {
                window = seq.first(direction_1d);
            }
        }
        if (window == nullptr) {
            logger.critical("%s", "Can't switch to window, sequence not found");
        } else {
            target_id = *window;
        }
    }
    else if (direction_2d != planar::Direction::INVALID) {
        planar::Arrangement arng;
        if (converters::any_focused(floating)) {
            logger.warning("%s", "Floating switching is misbehaving right now!");
            arng = converters::as_arrangement(floating, planar::Relation::CENTER);
        } else {
            arng = converters::as_arrangement(tiled, planar::Relation::BORDER);
        }
        std::string const *window = arng.next(direction_2d);
        if (window == nullptr && wrap) {
            window = arng.first(direction_2d);
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
