#include "planar.hpp"
#include "utils/logging.hpp"

#include <algorithm>
#include <climits>
#include <functional>
#include <map>

extern logging::Logger logger;

namespace {

using namespace planar;

bool le(int a, int b) { return a <= b; }
bool ge(int a, int b) { return a >= b; }

namespace alignment {

struct Properties {
    typedef decltype(&Rect::left) FnTypePtr;
    FnTypePtr near;         ///< First segment side encountered going along a orientation
    FnTypePtr far;          ///< Second segment side encountered going along a orientation
    FnTypePtr axis;         ///< Axis to seek center alignment to
    bool (*comp)(int, int); ///< Side comparator for the orientation
    int nearest;            ///< Value for smallest value of the direction
};


std::map<planar::Relation, std::map<planar::Direction, Properties>> properties{
    // clang-format off
    // DIRECTION                 NEAR                      FAR                       AXIS                      CMP NEAREST
    // ------------------------------------------------------------------------------------------------------------------------------------------
    {planar::Relation::BORDER, {
     {planar::Direction::LEFT,  {&Rect::right,             &Rect::left,              &Rect::vertical_middle,   le}},
     {planar::Direction::UP,    {&Rect::bottom,            &Rect::top,               &Rect::horizontal_middle, le}},
     {planar::Direction::RIGHT, {&Rect::left,              &Rect::right,             &Rect::vertical_middle,   ge}},
     {planar::Direction::DOWN,  {&Rect::top,               &Rect::bottom,            &Rect::horizontal_middle, ge}}}},
    {planar::Relation::CENTER, {
     {planar::Direction::LEFT,  {&Rect::horizontal_middle, &Rect::horizontal_middle, &Rect::vertical_middle,   le}},
     {planar::Direction::UP,    {&Rect::vertical_middle,   &Rect::vertical_middle,   &Rect::horizontal_middle, le}},
     {planar::Direction::RIGHT, {&Rect::horizontal_middle, &Rect::horizontal_middle, &Rect::vertical_middle,   ge}},
     {planar::Direction::DOWN,  {&Rect::vertical_middle,   &Rect::vertical_middle,   &Rect::horizontal_middle, ge}}}}
    // clang-format on
};
} // namespace alignment

std::vector<Rect const *> closest_in_direction(std::vector<Rect const *> const &rects, int lowest, alignment::Properties const &prop) {
    // find minimal position "greater" than lowest
    int min_pos = prop.comp(INT_MIN, INT_MAX) ? INT_MIN : INT_MAX;
    for (auto const *rect : rects) {
        int near = (rect->*prop.near)();
        if (prop.comp(near, lowest)) {
            min_pos = prop.comp(min_pos, near) ? near : min_pos;
        }
    }

    // we filter out the ones that aren't among the closest
    std::vector<Rect const *> closest;
    for (auto const *rect : rects) {
        int near = (rect->*prop.near)();
        if (near == min_pos) {
            rect->dump();
            closest.push_back(rect);
        }
    }

    return closest;
}

std::vector<Rect const *> aligned_in_direction(std::vector<Rect const *> const &rects, int value, alignment::Properties const &prop) {
    int min = INT_MAX;
    for (auto const *rect : rects) {
        int axis = (rect->*prop.axis)();
        int distance = std::abs(value - axis);
        min = std::min(min, distance);
    }

    std::vector<Rect const *> closest;
    for (auto const *rect : rects) {
        rect->dump();
        int axis = (rect->*prop.axis)();
        int distance = std::abs(value - axis);
        if (distance == min) {
            closest.push_back(rect);
        }
    }

    return closest;
}

Window const *next_in_direction(std::vector<Rect const *> const &rects, int current, alignment::Properties const &prop) {
    if (rects.empty()) {
        return nullptr;
    }

    // we filter out the ones that we are not interested in at all
    int extent_of_current = (rects[current]->*prop.far)();
    int middle_of_current = (rects[current]->*prop.axis)();
    logger.debug("exent: %d, axis: %d", extent_of_current, middle_of_current);

    auto closest = closest_in_direction(rects, extent_of_current, prop);
    logger.debug("closest found: %u", closest.size());
    closest = aligned_in_direction(closest, middle_of_current, prop);
    logger.debug("aligned found: %u", closest.size());

    return closest.empty() ? nullptr : static_cast<Window const *>(closest[0]);
}

Window const *first_of_direction(std::vector<Rect const *> const &rects, int current, alignment::Properties const &prop) {
    if (rects.empty()) {
        return nullptr;
    }

    // we filter out the ones that we are not interested in at all
    int extent_of_current = prop.comp(INT_MIN, INT_MAX) ? INT_MAX : INT_MIN;
    int middle_of_current = (rects[current]->*prop.axis)();

    auto closest = closest_in_direction(rects, extent_of_current, prop);
    logger.debug("closest found:%u", closest.size());
    closest = aligned_in_direction(closest, middle_of_current, prop);
    logger.debug("aligned found:%u", closest.size());

    return closest.empty() ? nullptr : static_cast<Window const *>(closest[0]);
}

} // namespace

namespace planar {

int Rect::left() const { return x; }

int Rect::right() const { return x + w; }

int Rect::top() const { return y; }

int Rect::bottom() const { return y + h; }

int Rect::vertical_middle() const { return y + (h / 2); }

int Rect::horizontal_middle() const { return x + (w / 2); }

void Rect::dump() const { logger.debug("{%d, %d, %d, %d}", x, y, w, h); }

Window::Window(Rect rect, std::string id) : Rect(rect), id(id) {}

void Window::dump() const { logger.info("{%d, %d, %d, %d, id:%s}", x, y, w, h, id.c_str()); }

Arrangement::Arrangement(std::vector<Window> _windows, size_t _current, Relation _relation)
    : windows(_windows), current(_current), relation(_relation) {
    std::transform(windows.begin(), windows.end(), std::back_inserter(rects),
                   [](Window const &window) { return static_cast<Rect const *>(&window); });
}

std::string const *Arrangement::next(Direction direction) const {
    auto const &prop = alignment::properties[relation][direction];
    auto *window = next_in_direction(rects, current, prop);
    return window == nullptr ? nullptr : &window->id;
}

std::string const *Arrangement::first(Direction direction) const {
    auto const &prop = alignment::properties[relation][direction];
    auto *window = first_of_direction(rects, current, prop);
    return window == nullptr ? nullptr : &window->id;
}

void Arrangement::dump() {
    logger.info("current: %u", current);
    for (auto window : windows) {
        window.dump();
    }
}

} // namespace planar
