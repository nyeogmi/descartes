#![feature(type_alias_impl_trait)]
mod endless_grid;
mod grid;
mod points_in;

pub use endless_grid::EndlessGrid;
pub use grid::Grid;
pub use points_in::PointsIn;

#[cfg(test)]
mod tests {
    use euclid::{UnknownUnit, point2, rect};

    use crate::Grid;

    // NYEO NOTE: This is just two unit tests from Grid
    // TODO: Test new functionality too

    #[test]
    fn test_get_mut() {
        let mut grid: Grid<char, UnknownUnit> = Grid::new(rect(0, 0, 1, 1,), || 'a');
        let value = grid.get_mut(point2(0, 0)).unwrap();

        assert_eq!(&'a', value);
        *value = 'b';
        assert_eq!(&'b', value);
    }

    #[test]
    fn test_get_set() {
        let mut grid: Grid<char, UnknownUnit> = Grid::new(rect(0, 0, 5, 5), || 'a');

        assert_eq!(Some(&'a'), grid.get(point2(2, 3)));
        assert_eq!(Some(&'a'), grid.get(point2(3, 3)));
        assert_eq!(Some(&'a'), grid.get(point2(3, 4)));

        grid.set(point2(2, 3), 'b');
        grid.set(point2(3, 3), 'c');
        grid.set(point2(3, 4), 'd');

        assert_eq!(Some(&'b'), grid.get(point2(2, 3)));
        assert_eq!(Some(&'c'), grid.get(point2(3, 3)));
        assert_eq!(Some(&'d'), grid.get(point2(3, 4)));
    }
}
