
mod grid;
mod direction;
mod vector;

#[cfg(test)]
mod tests {
    use grid::Grid;

    use crate::read_input;

    use super::*;

    #[test]
    fn test_part_one_case_one() {
        let s = read_input("src/day16/example.txt");

        let grid = Grid::new(s);
        grid.print_grid();
    }
}