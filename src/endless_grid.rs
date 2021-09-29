use std::{collections::{BTreeMap, btree_map::Entry}};

// vendored from `gridd` package
// rewritten to use euclid types
// and rewritten to have a customizable rectangle boundary
// and to remove some unnecessary helpers (ex: transpose(), square())
use euclid::{Point2D, Rect, Size2D, rect};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct EndlessGrid<T, Space> {
    // TODO: Use a faster store for this?
    observed_rect: Option<Rect<isize, Space>>,
    data: BTreeMap<(isize, isize), T>,
}

impl<T, Space> EndlessGrid<T, Space> {
    pub fn new() -> Self {
        Self { observed_rect: None, data: BTreeMap::new() }
    }

    pub fn contains(&self, p: Point2D<isize, Space>) -> bool {
        match self.observed_rect {
            Some(o) => o.contains(p),
            None => false
        }
    }

    pub fn rect(&self) -> Rect<isize, Space> {
        // returns the smallest rect possible that contains all the set cells in the grid
        self.observed_rect.unwrap_or(rect(0, 0, 0, 0))
    }

    pub fn size(&self) -> Size2D<isize, Space> {
        self.rect().size
    }

    pub fn get(&self, p: Point2D<isize, Space>) -> Option<&T> {
        self.data.get(&(p.x, p.y))
    }

    pub fn get_mut(&mut self, p: Point2D<isize, Space>) -> Option<&mut T> {
        self.data.get_mut(&(p.x, p.y))
    }

    pub fn get_mut_defaulting(&mut self, p: Point2D<isize, Space>, default: impl Fn() -> T) -> &mut T {
        match self.data.entry((p.x, p.y)) {
            Entry::Vacant(v) => {
                v.insert(default())
            }
            Entry::Occupied(o) => {
                o.into_mut()
            }
        }
    }

    pub fn set(&mut self, p: Point2D<isize, Space>, new_val: T) {
        self.data.insert((p.x, p.y), new_val);
    }
}