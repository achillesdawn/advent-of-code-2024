mod grid;

pub fn day_8() {}

#[cfg(test)]
mod test {
    use grid::Grid;

    use crate::read_input;

    use super::*;

    #[test]
    fn test_example() {
        let s = read_input("src/day8/test.txt");
        let mut g = Grid::new(s);

        g.collect_signals();
    }
}
