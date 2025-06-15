#include "linear.hpp"
#include "utils/logging.hpp"

extern logging::Logger logger;

namespace linear {

Sequence::Sequence(std::vector<std::string> const &ids, size_t current)
    : items(ids), current(current) {}

std::string const *Sequence::next(Direction direction) const {
    int delta = direction == Direction::PREV ? -1 : +1;
    return (0 <= current + delta && current + delta < items.size())
               ? &items[current + delta]
               : nullptr;
}

std::string const *Sequence::first(Direction direction) const {
    if (0 < items.size()) {
        return direction == Direction::PREV ? &items[items.size() - 1] : &items[0];
    }
    return nullptr;
}

std::string const *Sequence::operator[](size_t index) const {
    return index < items.size() ? &items[index] : nullptr;
}

void Sequence::dump() const {
    logger.debug("current:%u", current);
    for (auto item : items) {
        logger.debug("{id:%s}", item.c_str());
    }
}
} // namespace linears
