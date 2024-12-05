fn read_input() -> String {
    let mut current_dir = std::env::current_dir().unwrap();

    current_dir.push("src/day4/input.txt");

    let s = std::fs::read_to_string(current_dir).expect("could not find input.txt");

    s
}

mod grid;
use std::time::Instant;

use grid::{Direction, Grid};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

fn get_opposite_diagonal(diagonal: &Diagonal) -> Diagonal {
    match diagonal.direction {
        Direction::UpRight => Diagonal {
            x: diagonal.x + 2,
            y: diagonal.y - 2,
            direction: Direction::DownLeft,
        },
        Direction::DownLeft => Diagonal {
            x: diagonal.x - 2,
            y: diagonal.y + 2,
            direction: Direction::UpRight,
        },
        Direction::UpLeft => Diagonal {
            x: diagonal.x - 2,
            y: diagonal.y - 2,
            direction: Direction::DownRight,
        },
        Direction::DownRight => Diagonal {
            x: diagonal.x + 2,
            y: diagonal.y + 2,
            direction: Direction::UpLeft,
        },
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

    for y in 0..g.cols {
        for x in 0..g.rows {
            
            if g.get_unchecked(x, y) != 'M' {
                continue;
            }

            for direction in directions.iter() {
                let result = g.check_direction(x, y, direction);

                if !result {
                    continue;
                }

                let got_diagonal = Diagonal {
                    x,
                    y,
                    direction: direction.clone(),
                };

                let opposite_diagonal = get_opposite_diagonal(&got_diagonal);

                if g.diagonal_set.contains(&got_diagonal)
                    || g.diagonal_set.contains(&opposite_diagonal)
                {
                    continue;
                }

                let (first_diagonal, second_diagonal) = diagonals(x, y, direction);

                let check = g.check_diagonal(&first_diagonal);

                if check {
                    g.diagonal_set.insert(got_diagonal);
                    g.diagonal_set.insert(opposite_diagonal);

                    println!(
                        "Found X-MAS, x: {}, y: {}, direction: {} and x:{} y:{} direction: {}",
                        x,
                        y,
                        direction,
                        first_diagonal.x,
                        first_diagonal.y,
                        first_diagonal.direction
                    );
                    count += 1;
                    continue;
                }

                let check = g.check_diagonal(&second_diagonal);

                if check {
                    g.diagonal_set.insert(got_diagonal);
                    g.diagonal_set.insert(opposite_diagonal);

                    println!(
                        "Found X-MAS, x: {}, y: {}, direction: {} and x:{} y:{} direction: {}",
                        x,
                        y,
                        direction,
                        second_diagonal.x,
                        second_diagonal.y,
                        second_diagonal.direction
                    );
                    count += 1;
                    continue;
                }
            }
        }
    }
    count
}

pub fn day4_problem_one() {
    let s = read_input();

    let a = Grid::new(s.to_owned());
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

    #[ignore]
    #[test]
    fn case_two() {
        let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let a = Grid::new(s.to_owned());
        let count = search(a);
        assert_eq!(count, 9);
    }
}
