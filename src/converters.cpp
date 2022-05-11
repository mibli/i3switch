#include "converters.hpp"
#include "utils/logging.hpp"

extern logging::Logger logger;

namespace converters {
namespace {

std::string getId(json node) {
    return std::to_string(node["id"].get<int64_t>());
}

Window toWindow(json const &node) {
    json const &rect = node["rect"];
    bool floating = node["type"] == "floating_con";
    bool focused = floating ? node["nodes"][0]["focused"] : node["focused"];
    std::string id = floating ? getId(node["nodes"][0]) : getId(node);
    return {id, rect["x"], rect["y"], rect["width"], rect["height"], focused, floating};
}

planar::Window toPlanar(Window const &window) {
    return {{window.x, window.y, window.w, window.h}, window.id};
}

} // namespace

std::vector<json> visible_nodes(json node) {
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
        auto &floating_nodes = node["floating_nodes"];
        result.insert(result.end(), floating_nodes.begin(), floating_nodes.end());
        for (auto &subnode : node["nodes"]) {
            auto leaves = visible_nodes(subnode);
            result.insert(result.end(), leaves.begin(), leaves.end());
        }
        return result;
    } else if (layout == "tabbed" or layout == "stacking") {
        int64_t focus_id = node["focus"][0];
        for (auto &subnode : node["nodes"]) {
            if (subnode["id"] == focus_id) {
                return visible_nodes(subnode);
            }
        }
    } else if (layout == "dockarea") {
        return {};
    } else {
        logger.critical("Unsupported layout:%s found for id: %d",
                        layout.c_str(), getId(node));
    }
    return {};
}

std::vector<Window> to_windows(std::vector<json> const &nodes) {
    std::vector<Window> windows;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(windows), &toWindow);
    return windows;
}

std::vector<Window> floating_windows(std::vector<Window> const &windows) {
    std::vector<Window> result;
    std::copy_if(windows.begin(), windows.end(), std::back_inserter(result),
                 [](Window const &window) { return window.floating; });
    return result;
}

std::vector<Window> tiled_windows(std::vector<Window> const &windows) {
    std::vector<Window> result;
    std::copy_if(windows.begin(), windows.end(), std::back_inserter(result),
                 [](Window const &window) { return not window.floating; });
    return result;
}

bool any_focused(const std::vector<Window> &windows) {
    auto it = std::find_if(windows.begin(), windows.end(), [](const Window &window){ return window.focused; });
    return it != windows.end();
}

planar::Arrangement visible_grid(const std::vector<Window> &nodes) {
    std::vector<planar::Window> windows;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(windows),
                   &toPlanar);
    auto it = std::find_if(nodes.begin(), nodes.end(), [](Window const &window) { return window.focused; });
    if (it == nodes.end()) {
        logger.warning("No focused node found out of %lu", nodes.size());
        it = nodes.begin();
    }
    size_t index = std::distance(nodes.begin(), it);
    return planar::Arrangement(std::move(windows), index, planar::Relation::BORDER);
}

json find_deepest_focused_tabbed(json node) {
    logger.debug("Node iterated id:%s type:%s layout:%s",
                 getId(node).c_str(),
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
    logger.debug("Node iterated id:%s type:%s layout:%s",
                 getId(node).c_str(),
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

linear::Sequence available_tabs(json node) {
    node = find_deepest_focused_tabbed(node);
    json nodes = node["nodes"];

    std::vector<json> leaves;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(leaves),
                   find_deepest_focused);

    std::vector<std::string> tabs;
    std::transform(leaves.begin(), leaves.end(), std::back_inserter(tabs), &getId);

    int64_t focus_id = nodes.empty() ? 0 : node["focus"][0].get<int64_t>();
    auto it = std::find_if(
        nodes.begin(), nodes.end(),
        [focus_id](json const &subnode) { return subnode["id"] == focus_id; });
    size_t index = std::distance(nodes.begin(), it);
    return {tabs, index};
}

} // namespace converters
