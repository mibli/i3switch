#include "converters.hpp"
#include "utils/logging.hpp"

extern logging::Logger logger;

namespace converters {
namespace {

template <typename Type> Type to(json const &node);

template <> grid::Window to<grid::Window>(json const &node) {
    json rect = node["rect"];
    return {{rect["x"], rect["y"], rect["width"], rect["height"]},
            std::to_string(node["id"].get<int64_t>())};
}

template <> tabs::Tab to<tabs::Tab>(json const &node) {
    return {std::to_string(node["id"].get<int64_t>())};
}

} // namespace

std::vector<json> find_visible_children(json node) {
    logger.debug("Node iterated id:%ld type:%s layout:%s",
                 node["id"].get<int64_t>(),
                 node["type"].get<std::string>().c_str(),
                 node["layout"].get<std::string>().c_str());
    if (node["nodes"].empty() and node["type"] == "con") {
        if (node["rect"]["width"] == 0 or node["rect"]["height"] == 0) {
            return {};
        }
        return {node};
    }
    std::string layout = node["layout"];
    if (layout == "splith" or layout == "splitv" or layout == "output") {
        std::vector<json> result;
        for (auto subnode : node["nodes"]) {
            auto leaves = find_visible_children(subnode);
            result.insert(result.end(), leaves.begin(), leaves.end());
        }
        return result;
    } else if (layout == "tabbed" or layout == "stacking") {
        int64_t focus_id = node["focus"][0];
        for (auto subnode : node["nodes"]) {
            if (subnode["id"] == focus_id) {
                return find_visible_children(subnode);
            }
        }
    } else if (layout == "dockarea") {
        return {};
    } else {
        logger.critical("Unsupported layout:%s found for id: %d",
                        layout.c_str(), node["id"].get<int>());
    }
    return {};
}

grid::Grid visible_grid(json node) {
    auto nodes = find_visible_children(node);
    std::vector<grid::Window> windows;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(windows),
                   &to<grid::Window>);
    auto it = std::find_if(nodes.begin(), nodes.end(), [](json const &subnode) {
        return subnode["focused"] == true;
    });
    if (it == nodes.end()) {
        logger.warning("No focused node found out of %lu", nodes.size());
        it = nodes.begin();
    }
    size_t index = std::distance(nodes.begin(), it);
    return grid::Grid(std::move(windows), index);
}

json find_deepest_focused_tabbed(json node) {
    logger.debug("Node iterated id:%ld type:%s layout:%s",
                 node["id"].get<int64_t>(),
                 node["type"].get<std::string>().c_str(),
                 node["layout"].get<std::string>().c_str());
    if (node["focus"].empty()) {
        return {};
    }
    int64_t focus_id = node["focus"][0];
    for (auto subnode : node["nodes"]) {
        if (subnode["id"] != focus_id) {
            continue;
        }
        auto result = find_deepest_focused_tabbed(subnode);
        if (result != nullptr) {
            return result;
        }
        break;
    }
    std::string layout = node["layout"];
    if (layout == "tabbed" or layout == "stacking") {
        return node;
    }
    return {};
}

json find_deepest_focused(json node) {
    logger.debug("Node iterated id:%ld type:%s layout:%s",
                 node["id"].get<int64_t>(),
                 node["type"].get<std::string>().c_str(),
                 node["layout"].get<std::string>().c_str());
    if (node["focus"].empty()) {
        return node;
    }
    int64_t focus_id = node["focus"][0];
    for (auto subnode : node["nodes"]) {
        if (subnode["id"] != focus_id) {
            continue;
        }
        auto result = find_deepest_focused(subnode);
        if (result != nullptr) {
            return result;
        }
        break;
    }
    return node;
}

tabs::Tabs available_tabs(json node) {
    node = find_deepest_focused_tabbed(node);
    json nodes = node["nodes"];

    std::vector<json> leaves;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(leaves),
                   find_deepest_focused);

    std::vector<tabs::Tab> tabs;
    std::transform(leaves.begin(), leaves.end(), std::back_inserter(tabs),
                   &to<tabs::Tab>);

    int64_t focus_id = nodes.empty() ? 0 : node["focus"][0].get<int64_t>();
    auto it = std::find_if(
        nodes.begin(), nodes.end(),
        [focus_id](json const &subnode) { return subnode["id"] == focus_id; });
    size_t index = std::distance(nodes.begin(), it);
    return {tabs, index};
}

} // namespace converters
