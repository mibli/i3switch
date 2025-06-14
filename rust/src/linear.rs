use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Next,
    Prev,
}

#[derive(Debug)]
pub struct Sequence {
    items: Vec<u64>,
    current: usize,
}

impl Sequence {
    pub fn new(items: Vec<u64>, current: usize) -> Self {
        Self { items, current }
    }

    pub fn next(&self, direction: Direction) -> Option<u64> {
        let current: usize = if direction == Direction::Prev { self.current.wrapping_sub(1) } else { self.current.wrapping_add(1) };
        if current < self.items.len() {
            return Some(self.items[current]);
        }
        None
    }

    pub fn first(&self, direction: Direction) -> Option<u64> {
        if !self.items.is_empty() {
            return if direction == Direction::Prev { self.items.last() } else { self.items.first() }.cloned();
        }
        None
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

impl Index<usize> for Sequence {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let seq = Sequence::new(vec![1, 2, 3, 4, 5], 0);
        assert_eq!(seq.next(Direction::Next), Some(2));
        assert_eq!(seq.next(Direction::Prev), None);
        assert_eq!(seq.first(Direction::Next), Some(1));
        assert_eq!(seq.first(Direction::Prev), Some(5));
        assert_eq!(seq.size(), 5);

        let seq = Sequence::new(vec![10, 20, 30], 1);
        assert_eq!(seq.next(Direction::Next), Some(30));
        assert_eq!(seq.next(Direction::Prev), Some(10));
        assert_eq!(seq.first(Direction::Next), Some(10));
        assert_eq!(seq.first(Direction::Prev), Some(30));
        assert_eq!(seq.size(), 3);

        let seq = Sequence::new(vec![100, 200, 300], 2);
        assert_eq!(seq.next(Direction::Next), None);
        assert_eq!(seq.next(Direction::Prev), Some(200));
        assert_eq!(seq.first(Direction::Next), Some(100));
        assert_eq!(seq.first(Direction::Prev), Some(300));
        assert_eq!(seq.size(), 3);

        let seq = Sequence::new(vec![], 0);
        assert_eq!(seq.next(Direction::Next), None);
        assert_eq!(seq.next(Direction::Prev), None);
        assert_eq!(seq.first(Direction::Next), None);
        assert_eq!(seq.first(Direction::Prev), None);
        assert_eq!(seq.size(), 0);
    }

    #[test]
    fn test_indexing() {
        let seq = Sequence::new(vec![10, 20, 30], 0);
        assert_eq!(seq[0], 10);
        assert_eq!(seq[1], 20);
        assert_eq!(seq[2], 30);

        let seq = Sequence::new(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(seq[0], 1);
        assert_eq!(seq[1], 2);
        assert_eq!(seq[2], 3);
        assert_eq!(seq[3], 4);
        assert_eq!(seq[4], 5);

        let seq = Sequence::new(vec![42], 0);
        assert_eq!(seq[0], 42);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn test_indexing_out_of_bounds() {
        let seq = Sequence::new(vec![], 0);
        let _ = seq[0]; // This should panic
    }
}
