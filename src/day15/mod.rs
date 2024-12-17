use crate::read_input;

mod grid;
use direction::Direction;
use grid::Grid;

mod direction;
mod vector;

fn parse_input(s: String) -> (Grid, Vec<Direction>) {
    let (first, second) = s.split_once("\n\n").unwrap();
    let grid = Grid::new(first.to_owned());

    let commands = second.chars().map(Direction::from).collect();

    grid.print_grid();

    (grid, commands)
}

pub fn main() {
    let s = read_input("src/day15/example1.txt");
    let (mut grid, commands) = parse_input(s);

    for (idx, command) in commands.into_iter().enumerate() {
        println!("Moving {command}");
        grid.move_towards(command);
        grid.print_grid();
        
    }
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_example() {}
}
