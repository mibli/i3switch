#pragma once
#include "planar.hpp"
#include "linear.hpp"

#include "utils/logging.hpp"

#include <nlohmann/json.hpp>

namespace converters {

using nlohmann::json;

struct Window {
    std::string id;
    int x, y, w, h;
    bool focused;
    bool floating;
    void log() {
        printf("<%s %dx%d+%d+%d %s>%s\n", id.c_str(), w, h, x, y,
               floating ? "floating" : "tiled", focused ? "*" : "");
    }
};

std::vector<json> visible_nodes(json node);
std::vector<Window> to_windows(std::vector<json> const &node);
std::vector<Window> floating_windows(std::vector<Window> const &windows);
std::vector<Window> tiled_windows(std::vector<Window> const &windows);
bool any_focused(std::vector<Window> const &windows);

planar::Arrangement visible_grid(std::vector<Window> const &windows);

linear::Sequence available_tabs(json node);

}
