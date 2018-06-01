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

json Tree::find_tabbed(json &haystack, json const &needle)
{
    assert(haystack != nullptr);
    assert(needle != nullptr);
    auto node = needle;
    while (node["layout"] != "tabbed")
    {
        node = find_parent_of(haystack, node);
        if (node == nullptr)
            break;
    }
    return node;
}

json Tree::get_focused_child(json &haystack, size_t depth)
{
    assert(haystack != nullptr);
    json node = haystack;
    for (; depth > 0; --depth)
    {
        if (node["focus"].empty())
            break;
        json id = node["focus"][0];
        for (auto &child : node["nodes"])
            if (child["id"] == id)
                node = child;
    }
    return node;
}

void Tree::print_node(json &parent, size_t level, std::string const &prefix)
{
    assert(parent != nullptr);
    for (size_t i=0; i < level; ++i)
        printf("%s", prefix.c_str());
    printf("%ld", parent["id"].get<uint64_t>());
    if (parent["focused"] == true)
        printf("*");
    printf("\n");
    for (json child : parent["nodes"])
    {
        print_node(child, level + 1, prefix);
    }
}

}
