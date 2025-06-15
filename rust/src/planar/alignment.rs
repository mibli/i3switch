/// This module provides functions to align rectangles in a 2D plane based on their properties and
/// specified directions.
///
/// ```
/// // TODO: Fix tests
/// use i3switch::planar::alignment::{get_properties, next_in_direction, Direction, Relation};
/// use i3switch::planar::rect::Rect;
///
/// let rects = vec![
///    Rect { x: 0, y: 0, w: 10, h: 10 },
///    Rect { x: 20, y: 0, w: 10, h: 10 },
///    Rect { x: 0, y: 20, w: 10, h: 10 },
///    Rect { x: 20, y: 20, w: 10, h: 10 },
///    Rect { x: 10, y: 10, w: 10, h: 10 },
/// ];
/// let rect_refs: Vec<&Rect> = rects.iter().collect();
/// let properties = get_properties(Relation::Border, Direction::Right);
/// let next = next_in_direction(&rect_refs, &rect_refs[0], &properties);
/// assert_eq!(next, Some(4));
/// ```

use crate::planar::Rect;
use crate::logging;

/// This enum is used to specify the direction in which the focus should be moved.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// The `Relation` enum defines how the movement should be interpreted in relation to the
/// rectangles in the 2D plane. It can either be based on the borders of the rectangles or
/// their centers.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    /// Movement that looks at the sides of the window and tries to find the closest window to the
    /// border center.
    Border,
    /// Movement that treats center of the window as the point of reference and tries to find the
    /// closest other's window center in a given direction.
    Center,
}

/// The `Properties` struct contains functions that define how to calculate the near extent,
/// far extent, axis extent, and comparison function for rectangles in a 2D plane.
#[allow(dead_code)]
pub struct Properties {
    near: fn(&Rect) -> i32,
    far:  fn(&Rect) -> i32,
    axis: fn(&Rect) -> i32,
    comp: fn(i32, i32) -> bool,
}

#[allow(dead_code)]
impl Properties {
    pub fn new(
        near: fn(&Rect) -> i32,
        far:  fn(&Rect) -> i32,
        axis: fn(&Rect) -> i32,
        comp: fn(i32, i32) -> bool,
    ) -> Self {
        Properties { near, far, axis, comp }
    }
}

/// Is less than or equal.
#[allow(dead_code)]
fn le(a: i32, b: i32) -> bool {
    a <= b
}

/// Is greater than or equal.
#[allow(dead_code)]
fn ge(a: i32, b: i32) -> bool {
    a >= b
}


/// The properties define how to calculate the near extent, far extent, axis extent, and comparison function for rectangles in a 2D plane.
#[allow(dead_code)]
pub fn get_properties(relation: Relation, direction: Direction) -> Properties {
    match (relation, direction) {
    //   RELATION          DIRECTION                            NEAR                     FAR                      AXIS                     COMP
        (Relation::Border, Direction::Left ) => Properties::new(Rect::right,             Rect::left,              Rect::vertical_middle,   le),
        (Relation::Border, Direction::Right) => Properties::new(Rect::left,              Rect::right,             Rect::vertical_middle,   ge),
        (Relation::Border, Direction::Up   ) => Properties::new(Rect::bottom,            Rect::top,               Rect::horizontal_middle, le),
        (Relation::Border, Direction::Down ) => Properties::new(Rect::top,               Rect::bottom,            Rect::horizontal_middle, ge),
        (Relation::Center, Direction::Left ) => Properties::new(Rect::horizontal_middle, Rect::horizontal_middle, Rect::vertical_middle,   le),
        (Relation::Center, Direction::Right) => Properties::new(Rect::horizontal_middle, Rect::horizontal_middle, Rect::vertical_middle,   ge),
        (Relation::Center, Direction::Up   ) => Properties::new(Rect::vertical_middle,   Rect::vertical_middle,   Rect::horizontal_middle, le),
        (Relation::Center, Direction::Down ) => Properties::new(Rect::vertical_middle,   Rect::vertical_middle,   Rect::horizontal_middle, ge),
    }
}

/// Finds the closest rectangle in a given direction based on the properties.
fn closest_in_direction<'a>(rects: &'a [&Rect], at_least: i32, properties: &Properties) -> Vec<&'a Rect> {
    // Find the rectangle that is closest to the specified near extent.
    let min_key: for<'b> fn(&'b i32) -> i32;
    if (properties.comp)(i32::MIN, i32::MAX) {
        min_key = |extent: &i32| -extent
    } else {
        min_key = |extent: &i32| *extent
    }
    // Filter the rectangles that match the near extent condition.
    let min_extent = rects.iter()
        .filter(|rect| (properties.comp)((properties.near)(rect), at_least))
        .map(|rect| (properties.near)(rect))
        .min_by_key(min_key);

    // Filter the rectangles that match the minimum extent found.
    match min_extent {
        Some(min) => rects.iter()
            .filter(|rect| (properties.near)(rect) == min)
            .map(|rect| *rect)
            .collect(),
        None => vec![],
    }
}

/// Finds the rectangles that are aligned in a given direction based on the properties.
fn aligned_in_direction<'a>(rects: &'a [&Rect], close_to: i32, properties: &Properties) -> Vec<&'a Rect> {
    // Find the rectangle that is closest to the specified axis extent.
    let min_distance = rects.iter()
        .map(|rect| ((properties.axis)(rect) - close_to).abs())
        .min();

    // Filter the rectangles that match the minimum distance found.
    match min_distance {
        Some(min) => rects.iter()
            .filter(|rect| ((properties.axis)(rect) - close_to).abs() == min)
            .map(|rect| *rect)
            .collect(),
        None => vec![],
    }
}

/// Finds the next rectangle in a given direction based on the properties.
#[allow(dead_code)]
pub fn next_in_direction<'a>(rects: &'a [&Rect], current: &Rect, properties: &Properties) -> Option<usize> {
    let at_least = (properties.far)(&current);
    let mut closest = closest_in_direction(rects, at_least, &properties);
    logging::debug!("Closest found: {:?} for extent: {}", closest.len(), at_least);

    if closest.iter().any(|rect| std::ptr::eq(*rect, current)) {
        logging::debug!("Current rectangle is in the closest set, looking for next.");

        let at_least = if (properties.comp)(i32::MIN, i32::MAX) { at_least - 1 } else { at_least + 1 };
        closest = closest_in_direction(rects, at_least, &properties);
        logging::debug!("Closest after safety margin: {:?} for extent: {}", closest.len(), at_least);
    }

    let axis = (properties.axis)(&current);
    let aligned = aligned_in_direction(&closest, axis, &properties);
    logging::debug!("Aligned found: {:?} for axis: {}", aligned.len(), axis);

    if let Some(next) = aligned.first() {
        rects.iter().position(|rect| std::ptr::eq(*rect, *next))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn first_of_direction<'a>(rects: &'a [&Rect], current: &Rect, properties: &Properties) -> Option<usize> {
    let at_least: i32 = if (properties.comp)(i32::MIN, i32::MAX) { i32::MAX } else { i32::MIN };
    let closest = closest_in_direction(rects, at_least, properties);
    logging::debug!("Closest found: {:?} for extent: {}", closest.len(), at_least);

    let axis = (properties.axis)(&current);
    let aligned = aligned_in_direction(&closest, axis, &properties);
    logging::debug!("Aligned found: {:?} for axis: {}", aligned.len(), axis);

    if let Some(first) = aligned.first() {
        rects.iter().position(|rect| std::ptr::eq(*rect, *first))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planar::Rect;

    // In this test we check if the properties for each relation and direction correctly describe
    // the extents and direction comparison for a rectangle with coordinates for the left top
    // corner and width and height dimensions.
    #[test]
    fn test_get_properties() {
        macro_rules! test_properties {
            ($relation:expr, $direction:expr, $near:expr, $far:expr, $axis:expr, $comp:expr) => {
                let properties = get_properties($relation, $direction);
                assert_eq!((properties.near)(&Rect { x: 0, y: 2, w: 10, h: 20 }), $near,
                    "Near extent mismatch for {:?}, {:?}", $relation, $direction);
                assert_eq!((properties.far) (&Rect { x: 0, y: 2, w: 10, h: 20 }), $far,
                    "Far extent mismatch for {:?}, {:?}", $relation, $direction);
                assert_eq!((properties.axis)(&Rect { x: 0, y: 2, w: 10, h: 20 }), $axis,
                    "Axis mismatch for {:?} in {:?}", $relation, $direction);
                assert_eq!((properties.comp)(5, 10), $comp,
                    "Comparison function mismatch for {:?}, {:?}", $relation, $direction);
            };
        }
        //               RELATION          DIRECTION         NEAR  FAR  AXIS  COMP
        test_properties!(Relation::Border, Direction::Left,   10,   0,   12,  true);
        test_properties!(Relation::Border, Direction::Right,   0,  10,   12,  false);
        test_properties!(Relation::Border, Direction::Up,     22,   2,    5,  true);
        test_properties!(Relation::Border, Direction::Down,    2,  22,    5,  false);
        test_properties!(Relation::Center, Direction::Left,    5,   5,   12,  true);
        test_properties!(Relation::Center, Direction::Right,   5,   5,   12,  false);
        test_properties!(Relation::Center, Direction::Up,     12,  12,    5,  true);
        test_properties!(Relation::Center, Direction::Down,   12,  12,    5,  false);
    }

    // -------------------------------------------------------------------------------------------
    // Tests for the alignment functions.
    // These tests check if the functions correctly find the closest, aligned, next, and first
    // rectangles in a given direction based on the properties.
    // The tests use a macro to reduce code duplication and ensure that the expected results match
    // the actual results.
    // The `rects` vector contains a set of rectangles with specific coordinates and dimensions.
    // The layout of the rectangles is as follows:
    //
    //        ┌─────────┐         ┌─────────┐
    //        │         │         │         │
    //        │    0    │         │    1    │
    //        │         │         │         │
    //        └─────────┼─────────┼─────────┘
    //                  │         │
    //                  │    4    │
    //                  │         │
    //        ┌─────────┼─────────┼─────────┐
    //        │         │         │         │
    //        │    2    │         │    3    │
    //        │         │         │         │
    //        └─────────┘         └─────────┘
    //
    // -------------------------------------------------------------------------------------------

    // In this test we expect the closest rectangles to be found in a given direction for each of
    // the rectangles and directions. The rectangle should be with closes near extent to the far
    // extent of the current rectangle.
    #[test]
    fn test_closest_in_direction() {
        macro_rules! test_closest {
            ($rects:expr, $at_least:expr, $expected:expr, $relation:expr, $direction:expr) => {
                let rect_refs: Vec<&Rect> = $rects.iter().collect();
                let properties = get_properties($relation, $direction);
                let closest = closest_in_direction(&rect_refs, $at_least, &properties);
                assert_eq!(closest.len(), $expected.len(),
                    "Number of rectangle mismatch for {:?}, {:?} and at_least: {}",
                    $relation, $direction, $at_least);
                for (i, rect) in closest.iter().enumerate() {
                    assert_eq!(*rect, $expected[i],
                        "Rectangle mismatch at index {} for {:?}, {:?} and at_least: {}",
                        i, $relation, $direction, $at_least);
                }
            };
        }
        let rects = vec![
            Rect { x:  0, y:  0, w: 10, h: 10 },
            Rect { x: 20, y:  0, w: 10, h: 10 },
            Rect { x:  0, y: 20, w: 10, h: 10 },
            Rect { x: 20, y: 20, w: 10, h: 10 },
            Rect { x: 10, y: 10, w: 10, h: 10 },
        ];
        //                 AT_LEAST  EXPECTED RECTANGLES     RELATION          DIRECTION
        test_closest!(rects,  0,        Vec::<&Rect>::new(), Relation::Border, Direction::Left);
        test_closest!(rects, 10, vec![&rects[0], &rects[2]], Relation::Border, Direction::Left);
        test_closest!(rects, 20,            vec![&rects[4]], Relation::Border, Direction::Left);
        test_closest!(rects,  0, vec![&rects[0], &rects[2]], Relation::Border, Direction::Right);
        test_closest!(rects, 10,            vec![&rects[4]], Relation::Border, Direction::Right);
        test_closest!(rects, 20, vec![&rects[1], &rects[3]], Relation::Border, Direction::Right);
        test_closest!(rects,  0,        Vec::<&Rect>::new(), Relation::Border, Direction::Up);
        test_closest!(rects, 10, vec![&rects[0], &rects[1]], Relation::Border, Direction::Up);
        test_closest!(rects, 20,            vec![&rects[4]], Relation::Border, Direction::Up);
        test_closest!(rects,  0, vec![&rects[0], &rects[1]], Relation::Border, Direction::Down);
        test_closest!(rects, 10,            vec![&rects[4]], Relation::Border, Direction::Down);
        test_closest!(rects, 20, vec![&rects[2], &rects[3]], Relation::Border, Direction::Down);
        test_closest!(rects,  0,        Vec::<&Rect>::new(), Relation::Center, Direction::Left);
        test_closest!(rects, 10, vec![&rects[0], &rects[2]], Relation::Center, Direction::Left);
        test_closest!(rects, 20,            vec![&rects[4]], Relation::Center, Direction::Left);
        test_closest!(rects,  0, vec![&rects[0], &rects[2]], Relation::Center, Direction::Right);
        test_closest!(rects, 10,            vec![&rects[4]], Relation::Center, Direction::Right);
        test_closest!(rects, 20, vec![&rects[1], &rects[3]], Relation::Center, Direction::Right);
        test_closest!(rects,  0,        Vec::<&Rect>::new(), Relation::Center, Direction::Up);
        test_closest!(rects, 10, vec![&rects[0], &rects[1]], Relation::Center, Direction::Up);
        test_closest!(rects, 20,            vec![&rects[4]], Relation::Center, Direction::Up);
        test_closest!(rects,  0, vec![&rects[0], &rects[1]], Relation::Center, Direction::Down);
        test_closest!(rects, 10,            vec![&rects[4]], Relation::Center, Direction::Down);
        test_closest!(rects, 20, vec![&rects[2], &rects[3]], Relation::Center, Direction::Down);
    }

    // In this test we expect the rectangles to be aligned in a given direction for each of the
    // rectangles and directions. The rectangles should be aligned based on the axis extent.
    #[test]
    fn test_aligned_in_direction() {
        macro_rules! test_aligned {
            ($rects:expr, $close_to:expr, $expected:expr, $relation:expr, $direction:expr) => {
                let rect_refs: Vec<&Rect> = $rects.iter().collect();
                let properties = get_properties($relation, $direction);
                let aligned = aligned_in_direction(&rect_refs, $close_to, &properties);
                assert_eq!(aligned.len(), $expected.len(),
                    "Number of aligned rectangles mismatch for {:?}, {:?} and close_to: {}",
                    $relation, $direction, $close_to);
                for (i, rect) in aligned.iter().enumerate() {
                    assert_eq!(*rect, $expected[i],
                        "Aligned rectangle mismatch at index {} for {:?}, {:?} and close_to: {}",
                        i, $relation, $direction, $close_to);
                }
            };
        }
        let rects = vec![
            Rect { x:  0, y:  0, w: 10, h: 10 }, // Center at ( 5,  5)
            Rect { x: 20, y:  0, w: 10, h: 10 }, // Center at (25,  5)
            Rect { x:  0, y: 20, w: 10, h: 10 }, // Center at ( 5, 25)
            Rect { x: 20, y: 20, w: 10, h: 10 }, // Center at (25, 25)
            Rect { x: 10, y: 10, w: 10, h: 10 }, // Center at (15, 15)
        ];
        //                 CLOSE_TO      EXPECTED RECTANGLES             RELATION          DIRECTION
        test_aligned!(rects,   0,            vec![&rects[0], &rects[1]], Relation::Border, Direction::Left);
        test_aligned!(rects,  10, vec![&rects[0], &rects[1], &rects[4]], Relation::Border, Direction::Left);
        test_aligned!(rects,  20, vec![&rects[2], &rects[3], &rects[4]], Relation::Border, Direction::Left);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[1]], Relation::Border, Direction::Right);
        test_aligned!(rects,  10, vec![&rects[0], &rects[1], &rects[4]], Relation::Border, Direction::Right);
        test_aligned!(rects,  20, vec![&rects[2], &rects[3], &rects[4]], Relation::Border, Direction::Right);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[2]], Relation::Border, Direction::Up);
        test_aligned!(rects,  10, vec![&rects[0], &rects[2], &rects[4]], Relation::Border, Direction::Up);
        test_aligned!(rects,  20, vec![&rects[1], &rects[3], &rects[4]], Relation::Border, Direction::Up);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[2]], Relation::Border, Direction::Down);
        test_aligned!(rects,  10, vec![&rects[0], &rects[2], &rects[4]], Relation::Border, Direction::Down);
        test_aligned!(rects,  20, vec![&rects[1], &rects[3], &rects[4]], Relation::Border, Direction::Down);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[1]], Relation::Center, Direction::Left);
        test_aligned!(rects,  10, vec![&rects[0], &rects[1], &rects[4]], Relation::Center, Direction::Left);
        test_aligned!(rects,  20, vec![&rects[2], &rects[3], &rects[4]], Relation::Center, Direction::Left);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[1]], Relation::Center, Direction::Right);
        test_aligned!(rects,  10, vec![&rects[0], &rects[1], &rects[4]], Relation::Center, Direction::Right);
        test_aligned!(rects,  20, vec![&rects[2], &rects[3], &rects[4]], Relation::Center, Direction::Right);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[2]], Relation::Center, Direction::Up);
        test_aligned!(rects,  10, vec![&rects[0], &rects[2], &rects[4]], Relation::Center, Direction::Up);
        test_aligned!(rects,  20, vec![&rects[1], &rects[3], &rects[4]], Relation::Center, Direction::Up);
        test_aligned!(rects,   0,            vec![&rects[0], &rects[2]], Relation::Center, Direction::Down);
        test_aligned!(rects,  10, vec![&rects[0], &rects[2], &rects[4]], Relation::Center, Direction::Down);
        test_aligned!(rects,  20, vec![&rects[1], &rects[3], &rects[4]], Relation::Center, Direction::Down);
    }

    // In this test we expect the closest rectangles to be found in a given for each of the
    // rectangles and directions. The rectangle should be closest first by the near extent
    // and then the most aligned rectangle with the same axis extent, should be returned.
    // The result should not return the current rectangle, but the next closest one or none if
    // there is no such rectangle.
    #[test]
    fn test_next_in_direction() {
        macro_rules! test_next {
            ($rects:expr, $current:expr, $expected:expr, $relation:expr, $direction:expr) => {
                let rect_refs: Vec<&Rect> = $rects.iter().collect();
                let properties = get_properties($relation, $direction);
                let next = next_in_direction(&rect_refs, $current, &properties);
                assert_eq!(next, $expected,
                    "Next rectangle mismatch for {:?}, {:?} and current: {:?}",
                    $relation, $direction, $current);
            };
        }
        let rects = vec![
            Rect { x:  0, y:  0, w: 10, h: 10 }, // Center at ( 5,  5)
            Rect { x: 20, y:  0, w: 10, h: 10 }, // Center at (25,  5)
            Rect { x:  0, y: 20, w: 10, h: 10 }, // Center at ( 5, 25)
            Rect { x: 20, y: 20, w: 10, h: 10 }, // Center at (25, 25)
            Rect { x: 10, y: 10, w: 10, h: 10 }, // Center at (15, 15)
        ];
        //                CURRENT   EXPECTED RELATION          DIRECTION
        test_next!(rects, &rects[0],    None, Relation::Border, Direction::Left);
        test_next!(rects, &rects[1], Some(4), Relation::Border, Direction::Left);
        test_next!(rects, &rects[2],    None, Relation::Border, Direction::Left);
        test_next!(rects, &rects[3], Some(4), Relation::Border, Direction::Left);
        test_next!(rects, &rects[4], Some(0), Relation::Border, Direction::Left);
        test_next!(rects, &rects[0], Some(4), Relation::Border, Direction::Right);
        test_next!(rects, &rects[1],    None, Relation::Border, Direction::Right);
        test_next!(rects, &rects[2], Some(4), Relation::Border, Direction::Right);
        test_next!(rects, &rects[3],    None, Relation::Border, Direction::Right);
        test_next!(rects, &rects[4], Some(1), Relation::Border, Direction::Right);
        test_next!(rects, &rects[0],    None, Relation::Border, Direction::Up);
        test_next!(rects, &rects[1],    None, Relation::Border, Direction::Up);
        test_next!(rects, &rects[2], Some(4), Relation::Border, Direction::Up);
        test_next!(rects, &rects[3], Some(4), Relation::Border, Direction::Up);
        test_next!(rects, &rects[4], Some(0), Relation::Border, Direction::Up);
        test_next!(rects, &rects[0], Some(4), Relation::Border, Direction::Down);
        test_next!(rects, &rects[1], Some(4), Relation::Border, Direction::Down);
        test_next!(rects, &rects[2],    None, Relation::Border, Direction::Down);
        test_next!(rects, &rects[3],    None, Relation::Border, Direction::Down);
        test_next!(rects, &rects[4], Some(2), Relation::Border, Direction::Down);
        test_next!(rects, &rects[0],    None, Relation::Center, Direction::Left);
        test_next!(rects, &rects[1], Some(4), Relation::Center, Direction::Left);
        test_next!(rects, &rects[2],    None, Relation::Center, Direction::Left);
        test_next!(rects, &rects[3], Some(4), Relation::Center, Direction::Left);
        test_next!(rects, &rects[4], Some(0), Relation::Center, Direction::Left);
        test_next!(rects, &rects[0], Some(4), Relation::Center, Direction::Right);
        test_next!(rects, &rects[1],    None, Relation::Center, Direction::Right);
        test_next!(rects, &rects[2], Some(4), Relation::Center, Direction::Right);
        test_next!(rects, &rects[3],    None, Relation::Center, Direction::Right);
        test_next!(rects, &rects[4], Some(1), Relation::Center, Direction::Right);
        test_next!(rects, &rects[0],    None, Relation::Center, Direction::Up);
        test_next!(rects, &rects[1],    None, Relation::Center, Direction::Up);
        test_next!(rects, &rects[2], Some(4), Relation::Center, Direction::Up);
        test_next!(rects, &rects[3], Some(4), Relation::Center, Direction::Up);
        test_next!(rects, &rects[4], Some(0), Relation::Center, Direction::Up);
        test_next!(rects, &rects[0], Some(4), Relation::Center, Direction::Down);
        test_next!(rects, &rects[1], Some(4), Relation::Center, Direction::Down);
        test_next!(rects, &rects[2],    None, Relation::Center, Direction::Down);
        test_next!(rects, &rects[3],    None, Relation::Center, Direction::Down);
        test_next!(rects, &rects[4], Some(2), Relation::Center, Direction::Down);
    }

    // In this test we expect the first rectangle to be found in a given direction for each of the
    // rectangles and directions. The returned rectangles should be the first on the axis
    // in the given direction, and out of multiple rectangles, the one with the
    // one best aligned should be returned.
    #[test]
    fn test_first_of_direction() {
        macro_rules! test_first {
            ($rects:expr, $current:expr, $expected:expr, $relation:expr, $direction:expr) => {
                let rect_refs: Vec<&Rect> = $rects.iter().collect();
                let properties = get_properties($relation, $direction);
                let first = first_of_direction(&rect_refs, $current, &properties);
                assert_eq!(first, $expected,
                    "First rectangle mismatch for {:?}, {:?} and current: {:?}",
                    $relation, $direction, $current);
            };
        }
        let rects = vec![
            Rect { x:  0, y:  0, w: 10, h: 10 }, // Center at ( 5,  5)
            Rect { x: 20, y:  0, w: 10, h: 10 }, // Center at (25,  5)
            Rect { x:  0, y: 20, w: 10, h: 10 }, // Center at ( 5, 25)
            Rect { x: 20, y: 20, w: 10, h: 10 }, // Center at (25, 25)
            Rect { x: 10, y: 10, w: 10, h: 10 }, // Center at (15, 15)
        ];
        //                CURRENT   EXPECTED RELATION          DIRECTION
        test_first!(rects, &rects[0], Some(1), Relation::Border, Direction::Left);
        test_first!(rects, &rects[1], Some(1), Relation::Border, Direction::Left);
        test_first!(rects, &rects[2], Some(3), Relation::Border, Direction::Left);
        test_first!(rects, &rects[3], Some(3), Relation::Border, Direction::Left);
        test_first!(rects, &rects[4], Some(1), Relation::Border, Direction::Left);
        test_first!(rects, &rects[0], Some(0), Relation::Border, Direction::Right);
        test_first!(rects, &rects[1], Some(0), Relation::Border, Direction::Right);
        test_first!(rects, &rects[2], Some(2), Relation::Border, Direction::Right);
        test_first!(rects, &rects[3], Some(2), Relation::Border, Direction::Right);
        test_first!(rects, &rects[4], Some(0), Relation::Border, Direction::Right);
        test_first!(rects, &rects[0], Some(2), Relation::Border, Direction::Up);
        test_first!(rects, &rects[1], Some(3), Relation::Border, Direction::Up);
        test_first!(rects, &rects[2], Some(2), Relation::Border, Direction::Up);
        test_first!(rects, &rects[3], Some(3), Relation::Border, Direction::Up);
        test_first!(rects, &rects[4], Some(2), Relation::Border, Direction::Up);
        test_first!(rects, &rects[0], Some(0), Relation::Border, Direction::Down);
        test_first!(rects, &rects[1], Some(1), Relation::Border, Direction::Down);
        test_first!(rects, &rects[2], Some(0), Relation::Border, Direction::Down);
        test_first!(rects, &rects[3], Some(1), Relation::Border, Direction::Down);
        test_first!(rects, &rects[4], Some(0), Relation::Border, Direction::Down);
        test_first!(rects, &rects[0], Some(1), Relation::Center, Direction::Left);
        test_first!(rects, &rects[1], Some(1), Relation::Center, Direction::Left);
        test_first!(rects, &rects[2], Some(3), Relation::Center, Direction::Left);
        test_first!(rects, &rects[3], Some(3), Relation::Center, Direction::Left);
        test_first!(rects, &rects[4], Some(1), Relation::Center, Direction::Left);
        test_first!(rects, &rects[0], Some(0), Relation::Center, Direction::Right);
        test_first!(rects, &rects[1], Some(0), Relation::Center, Direction::Right);
        test_first!(rects, &rects[2], Some(2), Relation::Center, Direction::Right);
        test_first!(rects, &rects[3], Some(2), Relation::Center, Direction::Right);
        test_first!(rects, &rects[4], Some(0), Relation::Center, Direction::Right);
        test_first!(rects, &rects[0], Some(2), Relation::Center, Direction::Up);
        test_first!(rects, &rects[1], Some(3), Relation::Center, Direction::Up);
        test_first!(rects, &rects[2], Some(2), Relation::Center, Direction::Up);
        test_first!(rects, &rects[3], Some(3), Relation::Center, Direction::Up);
        test_first!(rects, &rects[4], Some(2), Relation::Center, Direction::Up);
        test_first!(rects, &rects[0], Some(0), Relation::Center, Direction::Down);
        test_first!(rects, &rects[1], Some(1), Relation::Center, Direction::Down);
        test_first!(rects, &rects[2], Some(0), Relation::Center, Direction::Down);
        test_first!(rects, &rects[3], Some(1), Relation::Center, Direction::Down);
        test_first!(rects, &rects[4], Some(0), Relation::Center, Direction::Down);
    }
}
