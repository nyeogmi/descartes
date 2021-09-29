use euclid::{Point2D, Rect, rect};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExpandingBounds<Space> {
    observed_rect: Option<Rect<isize, Space>>,
}

impl<Space> ExpandingBounds<Space> {
    pub fn new() -> ExpandingBounds<Space> {
        ExpandingBounds { observed_rect: None }
    }

    pub fn contains(&self, p: Point2D<isize, Space>) -> bool {
        match self.observed_rect {
            Some(o) => o.contains(p),
            None => false
        }
    }

    pub fn observe(&mut self, at: Point2D<isize, Space>) {
        self.observed_rect = match self.observed_rect {
            None => Some(rect(at.x, at.y, 1, 1)),
            Some(mut b) => {
                if b.contains(at) {
                    return
                }
                if at.x < b.min_x() {
                    b = rect(at.x, b.min_y(), b.max_x() - at.x, b.height())
                }
                if b.max_x() <= at.x {
                    b = rect(b.min_x(), b.min_y(), at.x - b.min_x() + 1, b.height())
                }
                if at.y < b.min_y() {
                    b = rect(b.min_x(), at.y, b.width(), b.max_y() - at.y)
                }
                if b.max_y() <= at.y {
                    b = rect(b.min_x(), b.min_y(), b.width(), at.y - b.min_y() + 1)
                }
                Some(b)
            }
        };
    }

    pub fn rect(&self) -> Rect<isize, Space> {
        // returns the smallest rect possible that contains all the set cells in the grid
        self.observed_rect.unwrap_or(rect(0, 0, 0, 0))
    }
}