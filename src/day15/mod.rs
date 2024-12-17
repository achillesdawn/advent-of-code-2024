use crate::read_input;

mod grid;
use direction::Direction;
use grid::Grid;

mod direction;
mod vector;

fn parse_input(s: String) -> (Grid, Vec<Direction>) {
    let (first, second) = s.split_once("\n\n").unwrap();
    let grid = Grid::new(first.to_owned());

    let commands = second
        .chars()
        .filter(|c| ['^', '<', '>', 'v'].contains(c))
        .map(Direction::from)
        .collect();

    grid.print_grid();

    (grid, commands)
}

pub fn main() {
    let s = read_input("src/day15/input.txt");
    let (mut grid, commands) = parse_input(s);

    for direction in commands.into_iter() {
        // println!("MOVING {direction}");
        grid.move_towards(direction);
        // grid.print_grid();
    }

    let sum_of_coords = grid.get_sum_coords();

    dbg!(sum_of_coords);
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_example_one() {
        let s = read_input("src/day15/example1.txt");
        let (mut grid, commands) = parse_input(s);

        for direction in commands.into_iter() {
            grid.move_towards(direction);
        }

        grid.print_grid();

        let sum_of_coords = grid.get_sum_coords();

        assert_eq!(sum_of_coords, 2028);
    }

    #[test]
    fn part_one_example_two() {
        let s = read_input("src/day15/example2.txt");
        let (mut grid, commands) = parse_input(s);

        for direction in commands.into_iter() {
            grid.move_towards(direction);
        }

        grid.print_grid();

        let sum_of_coords = grid.get_sum_coords();

        assert_eq!(sum_of_coords, 10092);
    }
}
