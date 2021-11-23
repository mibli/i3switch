#pragma once
#include "grid.hpp"
#include "tabs.hpp"

#include "utils/logging.hpp"

#include <nlohmann/json.hpp>

namespace converters {

using nlohmann::json;

std::vector<json> find_visible_children(json node);
grid::Grid visible_grid(json node);

json find_deepest_focused_tabbed(json node);
tabs::Tabs available_tabs(json node);

}
