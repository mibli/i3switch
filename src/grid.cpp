#include "grid.hpp"
#include "utils/logging.hpp"

#include <algorithm>
#include <climits>
#include <functional>
#include <map>

extern logging::Logger logger;

namespace {

using namespace grid;

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

std::map<std::pair<Direction2d, MovementType>, Properties> properties{
    // clang-format off
    // DIRECTION          TYPE                         NEAR                       FAR                       AXIS                      CMP NEAREST
    // ------------------------------------------------------------------------------------------------------------------------------------------
    {{Direction2d::LEFT,  MovementType::GRID_BASED},   {&Rect::right,             &Rect::left,              &Rect::vertical_middle,   le, INT_MAX}},
    {{Direction2d::UP,    MovementType::GRID_BASED},   {&Rect::bottom,            &Rect::top,               &Rect::horizontal_middle, le, INT_MAX}},
    {{Direction2d::RIGHT, MovementType::GRID_BASED},   {&Rect::left,              &Rect::right,             &Rect::vertical_middle,   ge, INT_MIN}},
    {{Direction2d::DOWN,  MovementType::GRID_BASED},   {&Rect::top,               &Rect::bottom,            &Rect::horizontal_middle, ge, INT_MIN}},
    {{Direction2d::LEFT,  MovementType::CENTER_BASED}, {&Rect::horizontal_middle, &Rect::horizontal_middle, &Rect::vertical_middle,   le, INT_MAX}},
    {{Direction2d::UP,    MovementType::CENTER_BASED}, {&Rect::vertical_middle,   &Rect::vertical_middle,   &Rect::horizontal_middle, le, INT_MAX}},
    {{Direction2d::RIGHT, MovementType::CENTER_BASED}, {&Rect::horizontal_middle, &Rect::horizontal_middle, &Rect::vertical_middle,   ge, INT_MIN}},
    {{Direction2d::DOWN,  MovementType::CENTER_BASED}, {&Rect::vertical_middle,   &Rect::vertical_middle,   &Rect::horizontal_middle, ge, INT_MIN}}
    // clang-format on
};
} // namespace alignment

std::vector<Rect const *> closest_in_direction(std::vector<Rect const *> const &rects, int lowest,
                                               Direction2d direction, MovementType movementType) {
    auto const &prop = alignment::properties[{direction, movementType}];

    int min_pos = INT_MAX;
    for (auto const *rect : rects) {
        rect->dump();
        if (prop.comp((rect->*prop.near)(), lowest)) {
            min_pos = std::min(min_pos, (rect->*prop.near)());
        }
    }

    // we filter out the ones that aren't among the closest
    std::vector<Rect const *> closest;
    for (auto const *rect : rects) {
        if ((rect->*prop.near)() == min_pos) {
            rect->dump();
            closest.push_back(rect);
        }
    }

    return closest;
}

std::vector<Rect const *> aligned_in_direction(std::vector<Rect const *> const &rects, int value,
                                               Direction2d direction, MovementType movementType) {
    auto const &prop = alignment::properties[{direction, movementType}];

    int min = INT_MAX;
    for (auto const *rect : rects) {
        int axis = (rect->*prop.axis)();
        int distance = std::abs(value - axis);
        min = std::min(min, distance);
    }

    std::vector<Rect const *> closest;
    for (auto const *rect : rects) {
        int axis = (rect->*prop.axis)();
        int distance = std::abs(value - axis);
        if (distance == min) {
            closest.push_back(rect);
        }
    }

    return closest;
}

Window const *next_in_direction(std::vector<Rect const *> const &rects, int current,
                                Direction2d direction, MovementType movementType) {
    if (rects.empty()) {
        return nullptr;
    }

    // we filter out the ones that we are not interested in at all
    auto const &prop = alignment::properties[{direction, movementType}];
    int extent_of_current = (rects[current]->*prop.far)();
    int middle_of_current = (rects[current]->*prop.axis)();

    auto closest = closest_in_direction(rects, extent_of_current, direction, movementType);
    logger.debug("closest found:%u", closest.size());
    closest = aligned_in_direction(closest, middle_of_current, direction, movementType);
    logger.debug("aligned found:%u", closest.size());

    return closest.empty() ? nullptr : static_cast<Window const *>(closest[0]);
}

Window const *first_of_direction(std::vector<Rect const *> const &rects, int current,
                                 Direction2d direction, MovementType movementType) {
    if (rects.empty()) {
        return nullptr;
    }

    auto const &prop = alignment::properties[{direction, movementType}];
    // we filter out the ones that we are not interested in at all
    int extent_of_current = prop.nearest;
    int middle_of_current = (rects[current]->*prop.axis)();

    auto closest = closest_in_direction(rects, extent_of_current, direction, movementType);
    logger.debug("closest found:%u", closest.size());
    closest = aligned_in_direction(closest, middle_of_current, direction, movementType);
    logger.debug("aligned found:%u", closest.size());

    return closest.empty() ? nullptr : static_cast<Window const *>(closest[0]);
}

} // namespace

namespace grid {

int Rect::left() const { return x; }

int Rect::right() const { return x + w; }

int Rect::top() const { return y; }

int Rect::bottom() const { return y + h; }

int Rect::vertical_middle() const { return y + (h / 2); }

int Rect::horizontal_middle() const { return x + (w / 2); }

void Rect::dump() const { logger.debug("{%d, %d, %d, %d}", x, y, w, h); }

Window::Window(Rect rect, std::string id) : Rect(rect), id(id) {}

void Window::dump() const { logger.info("{%d, %d, %d, %d, id:%s}", x, y, w, h, id.c_str()); }

Grid::Grid(std::vector<Window> _windows, size_t _current) : windows(_windows), current(_current) {
    std::transform(windows.begin(), windows.end(), std::back_inserter(rects),
                   [](Window const &window) { return static_cast<Rect const *>(&window); });
}

Window const *Grid::next(Direction2d direction, MovementType movementType) const {
    return static_cast<Window const *>(next_in_direction(rects, current, direction, movementType));
}

Window const *Grid::first(Direction2d direction, MovementType movementType) const {
    return static_cast<Window const *>(first_of_direction(rects, current, direction, movementType));
}

void Grid::dump() {
    logger.info("current: %u", current);
    for (auto window : windows) {
        window.dump();
    }
}

} // namespace grid
