#pragma once

#include <nlohmann/json.hpp>
#include <functional>

namespace i3
{

using nlohmann::json;

class Tree
{
public:
    json root;

    Tree(json root);
    Tree(Tree const &) = delete;
    ~Tree() = default;

    //matchers
    static bool is_focused(json const &obj);
    static bool is_parent_of(json const &haystack, json const &needle);

    //methods for using without the root
    static json find_where(json &haystack, std::function<bool (json const &)> matcher);
    static json find_focused(json &haystack);
    static json find_parent_of(json &haystack, json const &needle);
    static json find_tabbed(json &haystack, json const &needle);
    static json get_focused_child(json &haystack, size_t depth = SIZE_MAX);
    static json get_next_child(json &container);
    static json get_prev_child(json &container);
    static void print_node(json &parent, size_t level = 0, std::string const &prefix = "  ");

    //bindings for the root
    json find_where(std::function<bool(json const &)> matcher)
    {
        return find_where(root, matcher);
    }
    json find_focused()
    {
        return find_focused(root);
    }
    json find_parent_of(json const &element)
    {
        return find_parent_of(root, element);
    }
    json find_tabbed(json const &element)
    {
        return find_tabbed(root, element);
    }
};

}
