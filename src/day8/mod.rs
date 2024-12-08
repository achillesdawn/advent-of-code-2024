use grid::Grid;

use crate::read_input;

mod grid;

pub fn day_8() {
    let s = read_input("src/day8/input.txt");
    let mut g = Grid::new(s);

    g.print_grid();
    g.collect_signals();
    g.calculate_antinodes();

    dbg!(g.get_distinct_positions());
}

#[cfg(test)]
mod test {
    use grid::Grid;

    use crate::read_input;

    use super::*;

    #[test]
    fn test_example() {
        let s = read_input("src/day8/test.txt");
        let mut g = Grid::new(s);

        g.print_grid();
        g.collect_signals();
        g.calculate_antinodes();

        assert_eq!(g.get_distinct_positions(), 14);
    }
}
