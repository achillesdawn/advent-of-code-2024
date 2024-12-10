use grid::Grid;

use crate::read_input;

mod grid;
mod vector;

pub fn day10_problem_1() {
    let s = read_input("src/day10/example_1.txt");

    let mut g = Grid::new(s);

    g.print_grid();
    g.collect_trailheads();
}

#[cfg(test)]
mod test {
    use crate::read_input;

    use super::grid::Grid;

    #[test]
    fn test_example_one() {
        let s = read_input("src/day10/example_1.txt");

        let mut g = Grid::new(s);

        g.print_grid();

        g.collect_trailheads();
    }
}
