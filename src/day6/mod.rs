use grid::Grid;

use crate::read_input;

pub fn day_6_problem_one() {
    let s = read_input("src/day6/input.txt");
    let mut grid = Grid::new(s);

    grid.run();

    dbg!(grid.get_distinct_positions());
}

mod grid;

#[cfg(test)]
mod test {
    use grid::Grid;

    use crate::read_input;

    use super::*;

    #[test]
    fn test_start_pos() {
        let s = read_input("src/day6/test.txt");
        let grid = Grid::new(s);

        assert_eq!((grid.x, grid.y), (4, 6));
    }

    #[test]
    fn test_moving() {
        let s = read_input("src/day6/test.txt");
        let mut grid = Grid::new(s);

        grid.run();

        assert_eq!(grid.get_distinct_positions(), 41);
    }
}
