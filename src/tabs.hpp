#pragma once
#include <string>
#include <vector>

namespace tabs {

enum class Direction {
    PREV,
    NEXT
};

struct Tab {
    std::string id;
};

class Tabs {
public:
    Tabs(std::vector<Tab> const &ids, size_t current);

    Tab const *next(Direction direction) const;
    Tab const *first(Direction direction) const;

    Tab const *operator[](size_t index) const;

private:
    size_t current;
    std::vector<Tab> tabs;
};

}
