#pragma once
#include "direction.hpp"

#include <string>
#include <vector>

namespace linear {

enum class Direction { PREV, NEXT };

class Sequence {
public:
    Sequence(std::vector<std::string> const &ids, size_t current);

    std::string const *next(Direction direction) const;
    std::string const *first(Direction direction) const;

    std::string const *operator[](size_t index) const;

    void dump() const;

private:
    size_t current;
    std::vector<std::string> items;
};

}
