fn read_input() -> String {
    let mut current_dir = std::env::current_dir().unwrap();

    current_dir.push("src/day4/input.txt");

    let s = std::fs::read_to_string(current_dir).expect("could not find input.txt");

    s
}

mod grid;
use std::time::Instant;

use grid::Grid;

fn search(mut g: Grid) -> u32 {
    let directions = [
        grid::Direction::Up,
        grid::Direction::Down,
        grid::Direction::Right,
        grid::Direction::Left,
        grid::Direction::UpLeft,
        grid::Direction::UpRight,
        grid::Direction::DownLeft,
        grid::Direction::DownRight,
    ];

    let mut count = 0u32;

    for y in 0..g.cols {
        for x in 0..g.rows {
            let c = g.get_unchecked(x, y);
            if c != 'X' {
                continue;
            }

            for direction in directions.iter() {
                let result = g.check_direction(x, y, direction);
                if result {
                    // println!("FOUND AT {}, {} direction: {}", x, y, direction);
                    count += 1;
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

    #[test]
    fn case_one() {
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
        assert_eq!(count, 18);
    }
}
