#include "i3tree.hpp"

namespace i3
{

Tree::Tree(json root)
    : root(root)
{}

bool Tree::is_focused(json const &obj)
{
    return obj["focused"] == true;
}

bool Tree::is_parent_of(json const &haystack, json const &needle)
{
    uint64_t id = needle["id"];
    for (auto const &node : haystack["nodes"])
    {
        if (node["id"] == id)
            return true;
    }
    return false;
}

json Tree::find_where(json &haystack, std::function<bool (json const &)> matcher)
{
    if (matcher(haystack) == true)
    { return haystack; }
    for (auto &node : haystack["nodes"])
    {
        auto found = find_where(node, matcher);
        if (found != nullptr)
        { return found; }
    }
    return nullptr;
}

json Tree::find_focused(json &obj)
{
    return find_where(obj, is_focused);
}

json Tree::find_parent_of(json &haystack, json const &needle)
{
    return find_where(haystack, [&needle](json const &obj){ return is_parent_of(obj, needle); });
}

}
