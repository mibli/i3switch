#pragma once
#include "direction.hpp"

#include <string>
#include <vector>

namespace tabs {

struct Tab {
    std::string id;
};

class Tabs {
public:
    Tabs(std::vector<Tab> const &ids, size_t current);

    Tab const *next(Direction1d direction) const;
    Tab const *first(Direction1d direction) const;

    Tab const *operator[](size_t index) const;

    void dump() const;

private:
    size_t current;
    std::vector<Tab> tabs;
};

}
