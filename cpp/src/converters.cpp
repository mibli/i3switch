#include "converters.hpp"
#include "utils/logging.hpp"
#include <set>

extern logging::Logger logger;

namespace converters {
namespace {

enum class LayoutType {
    DIRECTIONAL,
    SWITCHING,
    SKIPPED,
    INVALID
};

std::string get_id(json node) {
    return std::to_string(node["id"].get<int64_t>());
}

Window to_window(json const &node) {
    json const &rect = node["rect"];
    bool floating = node["type"] == "floating_con";
    bool focused = floating ? node["nodes"][0]["focused"] : node["focused"];
    std::string id = floating ? get_id(node["nodes"][0]) : get_id(node);
    return {id, rect["x"], rect["y"], rect["width"], rect["height"], focused, floating};
}

planar::Window to_planar(Window const &window) {
    return {{window.x, window.y, window.w, window.h}, window.id};
}

bool is_focused(Window const &window) {
    return window.focused;
}

size_t focused_index(Windows const &windows) {
    auto it = std::find_if(windows.begin(), windows.end(), is_focused);
    if (it == windows.end()) {
        logger.warning("No focused node found out of %lu", windows.size());
        return 0; // we have to focus SOMETHING
    }
    return std::distance(windows.begin(), it);
}

bool is_end_node(json const &node) {
    return (node["nodes"].empty() and node["type"] == "con");
}

} // namespace

json focused_subnode(json &node) {
    auto &focus = node["focus"];
    if (is_end_node(node) or focus == nullptr or focus.empty()) {
        return nullptr;
    }
    int64_t focus_id = focus[0];
    auto &nodes = node["nodes"];
    auto it = std::find_if(nodes.begin(), nodes.end(), [focus_id](json const &node) { return node["id"] == focus_id; });
    if (it == nodes.end()) {
        return nullptr;
    }
    return *it;
}

LayoutType get_layout_type(json const &node) {
    std::string layout = node["layout"];
    if (node["type"] == "workspace" or node["type"] == "root") {
        return LayoutType::SWITCHING;
    }
    else if (layout == "splith" or layout == "splitv")
    {
        return LayoutType::DIRECTIONAL;
    }
    else if (layout ==  "tabbed" or layout == "stacked" or layout == "output")
    {
        return LayoutType::SWITCHING;
    }
    else if (layout == "dockarea")
    {
        return LayoutType::SKIPPED;
    }
    else {
        logger.critical("Unsupported layout:%s found for id: %s", layout.c_str(), get_id(node).c_str());
        return LayoutType::INVALID;
    }
}

bool is_invisible_node(json const &node) {
    return (node["rect"]["width"] == 0 or node["rect"]["height"] == 0);
}

std::vector<json> visible_nodes(json node) {
    logger.debug("V Node iterated id:%ld type:%s layout:%s",
                 node["id"].get<int64_t>(),
                 node["type"].get<std::string>().c_str(),
                 node["layout"].get<std::string>().c_str());
    // Does not have children
    if (is_end_node(node)) {
        if (is_invisible_node(node)) {
            return {};
        }
        return {node};
    }

    // Has children
    auto layout_type = get_layout_type(node);
    switch (layout_type) {
        case LayoutType::DIRECTIONAL:
            {
                std::vector<json> subnodes;

                // add subnodes
                for (auto &subnode : node["nodes"]) {
                    auto leaves = visible_nodes(subnode);
                    subnodes.insert(subnodes.end(), leaves.begin(), leaves.end());
                }
                return subnodes;
            }
        case LayoutType::SWITCHING:
            {
                std::vector<json> subnodes;

                // add floating nodes
                auto &floating_nodes = node["floating_nodes"];
                subnodes.insert(subnodes.end(), floating_nodes.begin(), floating_nodes.end());

                // add subnodes
                auto subnode = focused_subnode(node);
                if (subnode != nullptr) {
                    auto leaves = visible_nodes(subnode);
                    subnodes.insert(subnodes.end(), leaves.begin(), leaves.end());
                }
            }
        case LayoutType::SKIPPED:
        case LayoutType::INVALID:
            return {};
    }
    return {};
}

json find_deepest_focused_tabbed(json node) {
    logger.debug("T Node iterated id:%s type:%s layout:%s",
                 get_id(node).c_str(),
                 node["type"].get<std::string>().c_str(),
                 node["layout"].get<std::string>().c_str());
    auto subnode = focused_subnode(node);
    if (subnode == nullptr) {
        return {};
    }
    auto endnode = find_deepest_focused_tabbed(subnode);
    if (endnode != nullptr) {
        return endnode;
    }
    if (get_layout_type(node) == LayoutType::SWITCHING) {
        return node;
    }
    return {};
}

json find_deepest_focused(json node) {
    logger.debug("F Node iterated id:%s type:%s layout:%s",
                 get_id(node).c_str(),
                 node["type"].get<std::string>().c_str(),
                 node["layout"].get<std::string>().c_str());
    auto subnode = focused_subnode(node);
    if (subnode == nullptr) {
        return node;
    }
    auto endnode = find_deepest_focused(subnode);
    if (endnode != nullptr) {
        return endnode;
    }
    return subnode;
}

std::vector<json> available_tabs(json node) {
    node = find_deepest_focused_tabbed(node);
    if (node == nullptr) {
        return {};
    }
    json nodes = node["nodes"];
    if (nodes == nullptr or nodes.empty()) {
        return {};
    }

    std::vector<json> leaves;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(leaves),
                   find_deepest_focused);
    return leaves;
}


Windows floating(Windows const &windows) {
    static auto is_floating = [](Window const &window){ return window.floating; };
    std::vector<Window> result;
    std::copy_if(windows.begin(), windows.end(), std::back_inserter(result), is_floating);
    return result;
}

Windows tiled(Windows const &windows) {
    static auto is_tiled = [](Window const &window){ return not window.floating; };
    std::vector<Window> result;
    std::copy_if(windows.begin(), windows.end(), std::back_inserter(result), is_tiled);
    return result;
}

bool any_focused(const std::vector<Window> &windows) {
    static auto is_focused = [](Window const &window){ return window.focused; };
    auto it = std::find_if(windows.begin(), windows.end(), is_focused);
    return it != windows.end();
}

Windows to_windows(std::vector<json> const &nodes) {
    std::vector<Window> windows;
    std::transform(nodes.begin(), nodes.end(), std::back_inserter(windows), &to_window);
    return windows;
}

planar::Arrangement as_arrangement(const Windows &windows, planar::Relation relation) {
    size_t index = focused_index(windows);
    std::vector<planar::Window> items;
    std::transform(windows.begin(), windows.end(), std::back_inserter(items), &to_planar);
    return planar::Arrangement(std::move(items), index, planar::Relation::BORDER);
}

linear::Sequence as_sequence(const Windows &windows) {
    static auto get_id = [](Window const &window) { return window.id; };
    size_t index = focused_index(windows);
    std::vector<std::string> items;
    std::transform(windows.begin(), windows.end(), std::back_inserter(items), get_id);
    return linear::Sequence(std::move(items), index);
}

} // namespace converters
