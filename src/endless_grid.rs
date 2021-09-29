use std::{collections::{BTreeMap, btree_map::Entry}};

// vendored from `gridd` package
// rewritten to use euclid types
// and rewritten to have a customizable rectangle boundary
// and to remove some unnecessary helpers (ex: transpose(), square())
use euclid::{Point2D, Rect, Size2D};

use crate::expanding_bounds::{ExpandingBounds};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EndlessGrid<T, Space> {
    // TODO: Use a faster store for this?
    expanding_bounds: ExpandingBounds<Space>,
    data: BTreeMap<(isize, isize), T>,
    default: fn() -> T,
}

impl<T: Default, Space> Default for EndlessGrid<T, Space> {
    fn default() -> Self {
        Self::new(|| T::default())
    }
}

impl<T, Space> EndlessGrid<T, Space> {
    pub fn new(default: fn() -> T) -> Self {
        Self { 
            expanding_bounds: ExpandingBounds::new(), 
            data: BTreeMap::new(), 
            default,
        }
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

    pub fn get(&self, p: Point2D<isize, Space>) -> Option<&T> {
        self.data.get(&(p.x, p.y))
    }

    pub fn get_defaulting(&mut self, p: Point2D<isize, Space>) -> &T {
        self.get_mut_defaulting(p)
    }

    pub fn get_mut(&mut self, p: Point2D<isize, Space>) -> Option<&mut T> {
        self.data.get_mut(&(p.x, p.y))
    }

    pub fn get_mut_defaulting(&mut self, p: Point2D<isize, Space>) -> &mut T {
        match self.data.entry((p.x, p.y)) {
            Entry::Vacant(v) => {
                let m = v.insert((self.default)());
                self.expanding_bounds.observe(p);
                m
            }
            Entry::Occupied(o) => {
                o.into_mut()
            }
        }
    }

    pub fn set(&mut self, p: Point2D<isize, Space>, new_val: T) {
        self.expanding_bounds.observe(p);
        self.data.insert((p.x, p.y), new_val);
    }
}

#[cfg(test)]
mod tests {
    use euclid::{UnknownUnit, point2, rect};

    use crate::{EndlessGrid};

    #[test]
    fn test_endless_get_mut() {
        let mut grid: EndlessGrid<char, UnknownUnit> = EndlessGrid::new(|| 'a');
        let value = grid.get_mut_defaulting(point2(0, 0));

        assert_eq!(&'a', value);
        *value = 'b';
        assert_eq!(&'b', value);
    }

    #[test]
    fn test_endless_get_set() {
        let mut grid: EndlessGrid<char, UnknownUnit> = EndlessGrid::new(|| 'a');

        assert_eq!(&'a', grid.get_defaulting(point2(2, 3)));
        assert_eq!(&'a', grid.get_defaulting(point2(3, 3)));
        assert_eq!(&'a', grid.get_defaulting(point2(3, 4)));

        grid.set(point2(2, 3), 'b');
        grid.set(point2(3, 3), 'c');
        grid.set(point2(3, 4), 'd');

        assert_eq!(&'b', grid.get_defaulting(point2(2, 3)));
        assert_eq!(&'c', grid.get_defaulting(point2(3, 3)));
        assert_eq!(&'d', grid.get_defaulting(point2(3, 4)));
    }

    #[test]
    fn test_endless_rect() {
        let mut grid: EndlessGrid<char, UnknownUnit> = EndlessGrid::new(|| 'a');

        grid.set(point2(2, 3), 'b');
        grid.set(point2(3, 3), 'c');
        grid.set(point2(3, 4), 'd');

        assert_eq!(rect(2, 3, 2, 2), grid.rect())
    }
}