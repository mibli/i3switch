#pragma once
#include <functional>
#include <string>
#include <vector>

namespace planar {

enum class Direction { LEFT, UP, RIGHT, DOWN };

struct Rect {
    int bottom() const;
    int left() const;
    int right() const;
    int top() const;

    int vertical_middle() const;
    int horizontal_middle() const;

    int x, y, w, h;

    void dump() const;

    typedef int (Rect::*IntFn)() const;
};

struct Window : public Rect {
    Window(Rect rect, std::string id);
    std::string id;

    void dump() const;
};

enum class Relation {
    BORDER,  /** Movement that looks at the sides of the window and tries
                 to find  the closest to the border center. */
    CENTER   /** Movement that treats center of the window as the origin
                 and destination point and tries to find next in a direction. */
};

class Arrangement {
    public:
        Arrangement(std::vector<Window> windows, size_t current, Relation relation);

        std::string const *next(planar::Direction direction) const;
        std::string const *first(planar::Direction direction) const;

        void dump();

    private:
        Window const *closest_border(std::function<int(Rect const &)> near_extent_fn,
                                     std::function<int(Rect const &)> far_extent_fn,
                                     std::function<bool(int, int)> &comp) const;
        Window const *closest_border(std::function<int(Rect const &)> middle_fn) const;

    private:
        Relation relation;
        std::vector<Window> windows;
        std::vector<Rect const *> rects;
        size_t current;
};

} // namespace planar
