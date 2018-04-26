#pragma once

#include <nlohmann/json.hpp>
#include <functional>

namespace i3
{

using nlohmann::json;

class Tree
{
public:
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

    //bindings for the root
    json find_where(std::function<bool(json const &)> matcher) {
        return find_where(root, matcher);
    }
    json find_focused() { return find_focused(root); }
    json find_parent_of(json const &element) {
        return find_parent_of(root, element);
    }

   private:
    json root;
};

}
