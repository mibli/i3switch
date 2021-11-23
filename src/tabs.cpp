#include "tabs.hpp"

namespace tabs {

Tabs::Tabs(std::vector<Tab> const &ids, size_t current) :tabs(ids), current(current) {
}

Tab const *Tabs::next(Direction direction) const {
  int delta = direction == Direction::PREV ? -1 : +1;
  return (0 < current + delta && current + delta < tabs.size()) ? &tabs[current + delta] : nullptr;
}

Tab const *Tabs::first(Direction direction) const {
  if (0 < tabs.size()) {
    return direction == Direction::PREV ? &tabs[tabs.size() - 1] : &tabs[0];
  }
  return nullptr;
}

Tab const *Tabs::operator[](size_t index) const {
  return index < tabs.size() ? &tabs[index] : nullptr;
}
}
