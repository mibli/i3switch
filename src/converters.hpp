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

using Windows = std::vector<Window>;

std::vector<json> visible_nodes(json node);
std::vector<json> available_tabs(json node);

Windows floating(Windows const &windows);
Windows tiled(Windows const &windows);
bool any_focused(std::vector<Window> const &windows);

Windows to_windows(std::vector<json> const &node);
planar::Arrangement as_arrangement(Windows const &windows, planar::Relation relation);
linear::Sequence as_sequence(Windows const &windows);

}
