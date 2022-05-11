#pragma once
#include "grid.hpp"
#include "linear.hpp"

#include "utils/logging.hpp"

#include <nlohmann/json.hpp>

namespace converters {

using nlohmann::json;

std::vector<json> visible_nodes(json node);
bool any_focused(std::vector<json> const &node);

grid::Grid visible_grid(json node);
grid::Grid visible_grid(std::vector<json> const &node);

linear::Sequence available_tabs(json node);

}
