#[derive(Debug, PartialEq)]
pub struct GridPosition(pub usize, pub usize);

#[derive(Debug, PartialEq)]
pub struct GridSection {
    pub from: GridPosition,
    pub to: GridPosition,
}

pub struct Grid<T: Clone> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        if width < 1 || height < 1 {
            panic!("Grid width and height should both be larger than zero.");
        }

        Self {
            width,
            height,
            values: vec![default; width * height],
        }
    }

    fn validate_pos(&self, pos: &GridPosition) {
        if pos.0 >= self.width || pos.1 >= self.height {
            panic!("Invalid position.");
        }
    }

    fn validate_section(&self, section: &GridSection) {
        self.validate_pos(&section.from);
        self.validate_pos(&section.to);

        if section.from.0 > section.to.0 || section.from.1 > section.to.1 {
            panic!("From-coordinates should be smaller than to-coordinates.");
        }
    }

    fn index(&self, pos: &GridPosition) -> usize {
        let index = pos.1 * self.width + pos.0;
        index
    }

    fn index_or_panic(&self, pos: &GridPosition) -> usize {
        self.validate_pos(pos);
        self.index(pos)
    }

    pub fn get(&self, pos: &GridPosition) -> &T {
        let index = self.index_or_panic(pos);
        self.values.get(index).unwrap()
    }

    pub fn set(&mut self, new_value: T, at_pos: &GridPosition) {
        let coord_index = self.index_or_panic(at_pos);

        if let Some(value) = self.values.get_mut(coord_index) {
            *value = new_value
        } else {
            panic!("This should not happen because of the previous index check.");
        }
    }

    pub fn get_section(&self, section: &GridSection) -> Vec<&T> {
        self.validate_section(section);

        let section_width = section.to.0 - section.from.0 + 1;
        let section_height = section.to.1 - section.from.1 + 1;

        let mut values = Vec::with_capacity(section_width * section_height);

        for col in section.from.0..=section.to.0 {
            for row in section.from.1..=section.to.1 {
                values.push(self.get(&GridPosition(row, col)));
            }
        }

        values
    }

    pub fn set_section(&mut self, new_value: T, section: &GridSection) {
        self.validate_section(section);

        let mut row_start: usize;
        let mut row_end: usize;

        for row in section.from.1..=section.to.1 {
            row_start = self.index(&GridPosition(section.from.0, row));
            row_end = self.index(&GridPosition(section.to.0, row));
            self.values[row_start..=row_end].fill(new_value.clone());
        }
    }

    pub fn get_all(&self) -> &[T] {
        &self.values
    }

    pub fn mut_section<F: FnMut(&mut T) -> ()>(&mut self, section: &GridSection, mut func: F) {
        self.validate_section(section);

        let mut row_start: usize;
        let mut row_end: usize;

        for row in section.from.1..=section.to.1 {
            row_start = self.index(&GridPosition(section.from.0, row));
            row_end = self.index(&GridPosition(section.to.0, row));
            for v in self.values[row_start..=row_end].iter_mut() {
                func(v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_getting() {
        let grid = Grid::new(3, 3, "s");

        assert_eq!(*grid.get(&GridPosition(0, 0)), "s");
        assert_eq!(*grid.get(&GridPosition(1, 0)), "s");
        assert_eq!(*grid.get(&GridPosition(2, 0)), "s");

        assert_eq!(*grid.get(&GridPosition(0, 1)), "s");
        assert_eq!(*grid.get(&GridPosition(1, 1)), "s");
        assert_eq!(*grid.get(&GridPosition(2, 1)), "s");

        assert_eq!(*grid.get(&GridPosition(0, 2)), "s");
        assert_eq!(*grid.get(&GridPosition(1, 2)), "s");
        assert_eq!(*grid.get(&GridPosition(2, 2)), "s");

        let grid = Grid::new(1, 3, true);

        assert_eq!(*grid.get(&GridPosition(0, 0)), true);
        assert_eq!(*grid.get(&GridPosition(0, 1)), true);
        assert_eq!(*grid.get(&GridPosition(0, 2)), true);

        let grid = Grid::new(3, 1, vec![0]);

        assert_eq!(*grid.get(&GridPosition(0, 0)), vec![0]);
        assert_eq!(*grid.get(&GridPosition(1, 0)), vec![0]);
        assert_eq!(*grid.get(&GridPosition(2, 0)), vec![0]);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_01() {
        let grid = Grid::new(1, 1, true);
        assert_eq!(*grid.get(&GridPosition(0, 1)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_02() {
        let grid = Grid::new(1, 1, true);
        assert_eq!(*grid.get(&GridPosition(1, 0)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_03() {
        let grid = Grid::new(1, 1, true);
        assert_eq!(*grid.get(&GridPosition(1, 1)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_04() {
        let grid = Grid::new(2, 1, true);
        assert_eq!(*grid.get(&GridPosition(0, 1)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_05() {
        let grid = Grid::new(2, 1, true);
        assert_eq!(*grid.get(&GridPosition(2, 0)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_06() {
        let grid = Grid::new(2, 1, true);
        assert_eq!(*grid.get(&GridPosition(1, 1)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_07() {
        let grid = Grid::new(2, 1, true);
        assert_eq!(*grid.get(&GridPosition(2, 1)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_08() {
        let grid = Grid::new(1, 2, true);
        assert_eq!(*grid.get(&GridPosition(0, 2)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_09() {
        let grid = Grid::new(1, 2, true);
        assert_eq!(*grid.get(&GridPosition(1, 0)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_10() {
        let grid = Grid::new(1, 2, true);
        assert_eq!(*grid.get(&GridPosition(1, 1)), true);
    }

    #[test]
    #[should_panic]
    fn invalid_grid_getting_11() {
        let grid = Grid::new(1, 2, true);
        assert_eq!(*grid.get(&GridPosition(1, 2)), true);
    }

    #[test]
    fn set_values() {
        let mut grid = Grid::new(1, 1, 0);
        grid.set(1, &GridPosition(0, 0));
        assert_eq!(*grid.get(&GridPosition(0, 0)), 1);

        let mut grid = Grid::new(1, 2, 0);

        grid.set(2, &GridPosition(0, 1));

        assert_eq!(*grid.get(&GridPosition(0, 0)), 0);
        assert_eq!(*grid.get(&GridPosition(0, 1)), 2);

        grid.set(-42, &GridPosition(0, 0));

        assert_eq!(*grid.get(&GridPosition(0, 0)), -42);
        assert_eq!(*grid.get(&GridPosition(0, 1)), 2);
    }

    #[test]
    fn mut_from_to() {
        let mut grid = Grid::new(1, 1, true);
        grid.mut_section(
            &GridSection {
                from: GridPosition(0, 0),
                to: GridPosition(0, 0),
            },
            |v| *v = !*v,
        );
        assert_eq!(*grid.get(&GridPosition(0, 0)), false);
    }
}
