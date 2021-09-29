use crate::PointsIn;
use std::{collections::{BTreeMap}};

// vendored from `gridd` package
// rewritten to use euclid types
// and rewritten to have a customizable rectangle boundary
// and to remove some unnecessary helpers (ex: transpose(), square())
use euclid::{Point2D, Rect, Size2D, point2};

use crate::expanding_bounds::{ExpandingBounds};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CopyEndlessGrid<T: Copy, Space> {
    // TODO: Use a faster store for this?
    expanding_bounds: ExpandingBounds<Space>,
    data: BTreeMap<(isize, isize), T>,
    default: T,
}

impl<T: Copy+Default, Space> Default for CopyEndlessGrid<T, Space> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Copy, Space> CopyEndlessGrid<T, Space> {
    pub fn new(default: T) -> Self {
        Self { 
            expanding_bounds: ExpandingBounds::new(), 
            data: BTreeMap::new(), 
            default,
        }
    }

    pub fn iter(&self) -> impl '_+DoubleEndedIterator<Item=(Point2D<isize, Space>, T)> {
        isize::points_in(self.rect()).map(move |p| (p, self.get(p)))
    }

    // iterate over _populated_ items, as opposed to untouched ones
    pub fn iter_populated(&self) -> impl '_+DoubleEndedIterator<Item=(Point2D<isize, Space>, T)> {
        self.data.iter().map(|((x, y), v)|( point2(*x, *y), *v))
    }

    pub fn contains(&self, p: Point2D<isize, Space>) -> bool {
        self.expanding_bounds.contains(p)
    }

    pub fn rect(&self) -> Rect<isize, Space> {
        self.expanding_bounds.rect()
    }

    pub fn size(&self) -> Size2D<isize, Space> {
        self.rect().size
    }

    pub fn get(&self, p: Point2D<isize, Space>) -> T {
        self.data.get(&(p.x, p.y)).map(|x| *x).unwrap_or(self.default)
    }

    pub fn set(&mut self, p: Point2D<isize, Space>, new_val: T) {
        self.expanding_bounds.observe(p);
        self.data.insert((p.x, p.y), new_val);
    }
}

#[cfg(test)]
mod tests {
    use euclid::{UnknownUnit, point2, rect};

    use crate::{CopyEndlessGrid};

    #[test]
    fn test_endless_get_set() {
        let mut grid: CopyEndlessGrid<char, UnknownUnit> = CopyEndlessGrid::new('a');

        assert_eq!('a', grid.get(point2(2, 3)));
        assert_eq!('a', grid.get(point2(3, 3)));
        assert_eq!('a', grid.get(point2(3, 4)));

        grid.set(point2(2, 3), 'b');
        grid.set(point2(3, 3), 'c');
        grid.set(point2(3, 4), 'd');

        assert_eq!('b', grid.get(point2(2, 3)));
        assert_eq!('c', grid.get(point2(3, 3)));
        assert_eq!('d', grid.get(point2(3, 4)));
    }

    #[test]
    fn test_endless_rect() {
        let mut grid: CopyEndlessGrid<char, UnknownUnit> = CopyEndlessGrid::new('a');

        grid.set(point2(2, 3), 'b');
        grid.set(point2(3, 3), 'c');
        grid.set(point2(3, 4), 'd');

        assert_eq!(rect(2, 3, 2, 2), grid.rect())
    }
}