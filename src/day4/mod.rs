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

struct XmasCheck {
    x: usize,
    y: usize,
    direction: Direction,
}

fn search(mut g: Grid) -> u32 {
    let directions = [
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    let mut count = 0u32;

    for y in 0..g.cols {
        for x in 0..g.rows {
            let c = g.get_unchecked(x, y);
            if c != 'M' {
                continue;
            }

            for direction in directions.iter() {
                let result = g.check_direction(x, y, direction);
                if result {
                    // println!("FOUND AT {}, {} direction: {}", x, y, direction);
                    let (check1, check2) = match direction {
                        Direction::UpRight => (
                            XmasCheck {
                                x: x + 2,
                                y,
                                direction: Direction::UpLeft,
                            },
                            (XmasCheck {
                                x,
                                y: y - 2,
                                direction: Direction::DownRight,
                            }),
                        ),
                        Direction::UpLeft => (
                            XmasCheck {
                                x: x - 2,
                                y,
                                direction: Direction::UpRight,
                            },
                            (XmasCheck {
                                x,
                                y: y - 2,
                                direction: Direction::DownLeft,
                            }),
                        ),
                        Direction::DownRight => (
                            XmasCheck {
                                x,
                                y: y + 2,
                                direction: Direction::UpRight,
                            },
                            (XmasCheck {
                                x: x + 2,
                                y,
                                direction: Direction::DownLeft,
                            }),
                        ),
                        Direction::DownLeft => (
                            XmasCheck {
                                x,
                                y: y + 2,
                                direction: Direction::UpRight,
                            },
                            (XmasCheck {
                                x: x + 2,
                                y,
                                direction: Direction::DownLeft,
                            }),
                        ),
                        _ => panic!(),
                    };

                    let check = g.check_direction(x, y, new_direction1);
                    if check {
                        println!(
                            "Found X-MAS, x: {}, y: {}, direction: {} {}",
                            x, y, direction, new_direction1
                        );
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
