use grid::Grid;

use crate::read_input;

mod grid;
mod vector;

pub fn day10_problem_1() {
    let s = read_input("src/day10/input.txt");

    let mut g = Grid::new(s);

    g.print_grid();
    let trails = g.collect_trailheads();

    dbg!(trails.len());
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

        let trails = g.collect_trailheads();

        assert_eq!(trails.len(), 2)
    }

    #[test]
    fn test_example_two() {
        let s = read_input("src/day10/example_2.txt");

        let mut g = Grid::new(s);

        g.print_grid();

        let trails = g.collect_trailheads();

        assert_eq!(trails.len(), 4)
    }

    #[test]
    fn test_example_three() {
        let s = read_input("src/day10/example_3.txt");

        let mut g = Grid::new(s);

        g.print_grid();

        let trails = g.collect_trailheads();

        assert_eq!(trails.len(), 3)
    }

    #[test]
    fn test_example_four() {
        let s = read_input("src/day10/example_4.txt");

        let mut g = Grid::new(s);

        g.print_grid();

        let trails = g.collect_trailheads();

        assert_eq!(trails.len(), 36)
    }
}
