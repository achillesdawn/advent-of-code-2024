fn read_input() -> String {
    let mut current_dir = std::env::current_dir().unwrap();

    current_dir.push("src/day4/input.txt");

    let s = std::fs::read_to_string(current_dir).expect("could not find input.txt");

    s
}

mod grid;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use grid::{Direction, Grid};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Diagonal {
    x: usize,
    y: usize,
    direction: Direction,
}

fn diagonals(x: usize, y: usize, direction: &Direction) -> (Diagonal, Diagonal) {
    match direction {
        Direction::UpRight => (
            Diagonal {
                x: x + 2,
                y,
                direction: Direction::UpLeft,
            },
            (Diagonal {
                x,
                y: y - 2,
                direction: Direction::DownRight,
            }),
        ),
        Direction::UpLeft => (
            Diagonal {
                x: x - 2,
                y,
                direction: Direction::UpRight,
            },
            (Diagonal {
                x,
                y: y - 2,
                direction: Direction::DownLeft,
            }),
        ),
        Direction::DownRight => (
            Diagonal {
                x,
                y: y + 2,
                direction: Direction::UpRight,
            },
            (Diagonal {
                x: x + 2,
                y,
                direction: Direction::DownLeft,
            }),
        ),
        Direction::DownLeft => (
            Diagonal {
                x,
                y: y + 2,
                direction: Direction::UpRight,
            },
            (Diagonal {
                x: x + 2,
                y,
                direction: Direction::DownLeft,
            }),
        ),
        _ => panic!(),
    }
}

fn search(mut g: Grid) -> u32 {
    let directions = [
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    let mut count = 0u32;
    let mut checked = HashSet::new();

    for y in 0..g.cols {
        for x in 0..g.rows {
            let c = g.get_unchecked(x, y);
            if c != 'M' {
                continue;
            }

            for direction in directions.iter() {
                let result = g.check_direction(x, y, direction);
                if result {
                    let first_check = Diagonal {
                        x,
                        y,
                        direction: direction.clone(),
                    };

                    if checked.contains(&first_check) {
                        continue;
                    }

                    let (first_diagonal, second_diagonal) = diagonals(x, y, direction);

                    let mut checked_result;
                    if checked.contains(&first_diagonal) {
                        checked_result = false;
                    } else {
                        checked_result = g.check_direction(
                            first_diagonal.x,
                            first_diagonal.y,
                            &first_diagonal.direction,
                        );
                    }

                    if checked_result {
                        println!(
                            "Found X-MAS, x: {}, y: {}, direction: {} and x:{} y:{} direction: {}",
                            x,
                            y,
                            direction,
                            first_diagonal.x,
                            first_diagonal.y,
                            first_diagonal.direction
                        );
                        checked.insert(first_check);
                        checked.insert(first_diagonal);

                        count += 1;
                        continue;
                    }

                    if checked.contains(&second_diagonal) {
                        checked_result = false;
                    } else {
                        checked_result = g.check_direction(
                            second_diagonal.x,
                            second_diagonal.y,
                            &second_diagonal.direction,
                        );
                    }

                    if checked_result {
                        println!(
                            "Found X-MAS, x: {}, y: {}, direction: {} and x:{} y:{} direction: {}",
                            x,
                            y,
                            direction,
                            second_diagonal.x,
                            second_diagonal.y,
                            second_diagonal.direction
                        );
                        checked.insert(first_check);
                        checked.insert(second_diagonal);

                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn day4_problem_one() {
    let s = read_input();

    let a = Grid::new_reversed(s.to_owned());
    let now = Instant::now();
    let count = search(a);

    let elapsed = now.elapsed();
    dbg!(elapsed.as_millis());
    dbg!(count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn case_one() {
        let s = "MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX";

        let a = Grid::new(s.to_owned());
        let count = search(a);
        assert_eq!(count, 18);
    }

    #[test]
    fn case_two() {
        let s = "MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX";

        let a = Grid::new_reversed(s.to_owned());
        let count = search(a);
        assert_eq!(count, 9);
    }
}
