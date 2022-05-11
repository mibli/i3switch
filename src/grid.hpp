#pragma once
#include "direction.hpp"

#include <functional>
#include <string>
#include <vector>

namespace grid {

struct Rect {
    int bottom() const;
    int left() const;
    int right() const;
    int top() const;

    int vertical_middle() const;
    int horizontal_middle() const;

    int x;
    int y;
    int w;
    int h;

    void dump() const;

    typedef int (Rect::*IntFn)() const;
};

struct Window : public Rect {
    Window(Rect rect, std::string id);
    std::string id;

    void dump() const;
};

enum class MovementType {
    GRID_BASED,  /** Movement that looks at the sides of the window and tries
                     to find  the closest to the border center. */
    CENTER_BASED /** Movement that treats center of the window as the origin
                     and destination point and tries to find next in
                     a direction. */
};

class Grid {
  public:
    Grid(std::vector<Window> windows, size_t current);

    Window const *next(Direction2d direction, MovementType movementType) const;
    Window const *first(Direction2d direction, MovementType movementType) const;

    void dump();

  private:
    Window const *
    closest_border(std::function<int(Rect const &)> near_extent_fn,
                   std::function<int(Rect const &)> far_extent_fn,
                   std::function<bool(int, int)> &comp) const;
    Window const *
    closest_border(std::function<int(Rect const &)> middle_fn) const;

  private:
    std::vector<Window> windows;
    std::vector<Rect const *> rects;
    size_t current;
};

} // namespace grid
