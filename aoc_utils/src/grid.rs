use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors4(self) -> [Point; 4] {
        [
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
    }

    pub fn neighbors8(self) -> [Point; 8] {
        [
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x - 1, self.y - 1),
        ]
    }

    /// Convert (row, col) to Point, if you prefer that interpretation.
    pub fn from_rc(row: i32, col: i32) -> Self {
        Self { x: col, y: row }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

/// Helpers for working with 2D grids stored as Vec<Vec<T>>.
pub trait GridExt<T> {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn in_bounds(&self, p: Point) -> bool;
    fn get_point(&self, p: Point) -> Option<&T>;
    fn get_point_mut(&mut self, p: Point) -> Option<&mut T>;
}

impl<T> GridExt<T> for Vec<Vec<T>> {
    fn width(&self) -> i32 {
        self.first().map(|row| row.len() as i32).unwrap_or(0)
    }

    fn height(&self) -> i32 {
        self.len() as i32
    }

    fn in_bounds(&self, p: Point) -> bool {
        p.y >= 0 && p.y < self.height() && p.x >= 0 && p.x < self.width()
    }

    fn get_point(&self, p: Point) -> Option<&T> {
        if !self.in_bounds(p) {
            return None;
        }
        self.get(p.y as usize).and_then(|row| row.get(p.x as usize))
    }

    fn get_point_mut(&mut self, p: Point) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None;
        }
        self.get_mut(p.y as usize)
            .and_then(|row| row.get_mut(p.x as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // -------- Point tests --------

    #[test]
    fn point_new_sets_coordinates() {
        let p = Point::new(3, -5);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, -5);
    }

    #[test]
    fn manhattan_distance_basic() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.manhattan(b), 7);
        assert_eq!(b.manhattan(a), 7); // symmetry
    }

    #[test]
    fn manhattan_distance_with_negative_coords() {
        let a = Point::new(-2, -3);
        let b = Point::new(1, 1);
        // | -2 - 1 | + | -3 - 1 | = 3 + 4 = 7
        assert_eq!(a.manhattan(b), 7);
    }

    #[test]
    fn neighbors4_are_correct() {
        let p = Point::new(10, 20);
        let ns = p.neighbors4();
        assert_eq!(
            ns,
            [
                Point::new(11, 20),
                Point::new(9, 20),
                Point::new(10, 21),
                Point::new(10, 19)
            ]
        );
    }

    #[test]
    fn neighbors8_are_correct() {
        let p = Point::new(0, 0);
        let ns = p.neighbors8();
        assert_eq!(
            ns,
            [
                Point::new(1, 0),
                Point::new(-1, 0),
                Point::new(0, 1),
                Point::new(0, -1),
                Point::new(1, 1),
                Point::new(1, -1),
                Point::new(-1, 1),
                Point::new(-1, -1),
            ]
        );
    }

    #[test]
    fn from_rc_converts_row_col_to_xy() {
        let p = Point::from_rc(5, 7);
        // from_rc(row, col) => x = col, y = row
        assert_eq!(p, Point::new(7, 5));
    }

    #[test]
    fn debug_format_is_expected() {
        let p = Point::new(3, 4);
        assert_eq!(format!("{:?}", p), "(3, 4)");
    }

    #[test]
    fn point_is_hashable_and_uses_both_coords() {
        let mut set = HashSet::new();
        set.insert(Point::new(1, 2));
        assert!(set.contains(&Point::new(1, 2)));
        assert!(!set.contains(&Point::new(2, 1)));
    }

    // -------- GridExt tests --------

    #[test]
    fn width_and_height_on_non_empty_grid() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
    }

    #[test]
    fn width_and_height_on_empty_grid() {
        let grid: Vec<Vec<i32>> = Vec::new();
        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
    }

    #[test]
    fn in_bounds_works_for_valid_and_invalid_points() {
        let grid = vec![vec![0; 3]; 2]; // 2 rows, 3 cols
        // valid corners
        assert!(grid.in_bounds(Point::new(0, 0)));
        assert!(grid.in_bounds(Point::new(2, 1)));

        // just outside bounds
        assert!(!grid.in_bounds(Point::new(-1, 0)));
        assert!(!grid.in_bounds(Point::new(0, -1)));
        assert!(!grid.in_bounds(Point::new(3, 0))); // x == width
        assert!(!grid.in_bounds(Point::new(0, 2))); // y == height
    }

    #[test]
    fn get_point_returns_correct_reference() {
        let grid = vec![
            vec![10, 11, 12],
            vec![20, 21, 22],
        ];

        assert_eq!(grid.get_point(Point::new(0, 0)), Some(&10));
        assert_eq!(grid.get_point(Point::new(2, 1)), Some(&22));
        assert_eq!(grid.get_point(Point::new(3, 0)), None); // out of bounds
        assert_eq!(grid.get_point(Point::new(0, 2)), None); // out of bounds
    }

    #[test]
    fn get_point_mut_allows_mutation() {
        let mut grid = vec![
            vec!['a', 'b'],
            vec!['c', 'd'],
        ];

        if let Some(cell) = grid.get_point_mut(Point::new(1, 0)) {
            *cell = 'X';
        } else {
            panic!("Expected Some(&mut T) for in-bounds point");
        }

        assert_eq!(grid[0][1], 'X');
        // out of bounds still returns None
        assert!(grid.get_point_mut(Point::new(2, 0)).is_none());
    }
}
