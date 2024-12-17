use colored::Colorize;
use nalgebra::Vector2;

use crate::read_input;

#[derive(Debug)]
struct Robot {
    pos: Vector2<isize>,
    vel: Vector2<isize>,
}

const ROWS: isize = 101;
const COLS: isize = 103;

fn parse_vector(s: &str) -> Vector2<isize> {
    let (x, y) = s.split_once("=").unwrap().1.split_once(",").unwrap();

    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

    Vector2::new(x, y)
}

fn parse_input(s: String) -> Vec<Robot> {
    s.lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" ").unwrap();

            let pos = parse_vector(pos);
            let vel = parse_vector(vel);

            Robot { pos, vel }
        })
        .collect()
}

type Grid = [[usize; 101]; 103];

fn draw_grid(positions: Vec<Vector2<usize>>) -> Grid {
    let mut grid: Grid = [[0; 101]; 103];

    for position in positions.into_iter() {
        grid[position.y][position.x] += 1;
    }

    grid
}

fn print_grid(grid: Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let item = grid[y][x];
            if item == 0 {
                print!(".");
            } else {
                print!("{item}");
            }
        }
        println!();
    }
}

type CountResult = [[usize; 2]; 2];

fn count_grid(grid: Grid) -> CountResult {
    let mut counts = [[0usize; 2]; 2];

    let cols = grid.len();
    let rows = grid[0].len();

    let half_cols = cols / 2;
    let half_rows = rows / 2;

    let mut quadrant_x = 0usize;

    for y in 0..cols {
        for x in 0..rows {
            let item = grid[y][x];

            if x < half_rows {
                quadrant_x = 0;
            } else if x > half_rows {
                quadrant_x = 1;
            } else {
                continue;
            }

            if y < half_cols {
                counts[0][quadrant_x] += item;
            } else if y > half_cols {
                counts[1][quadrant_x] += item;
            } else {
                continue;
            }
        }
    }

    counts
}

fn calculate_step(robots: &Vec<Robot>, step: isize) {
    let mut positions: Vec<Vector2<usize>> = Vec::new();

    for robot in robots {
        let total_vel = robot.vel * step;
        let mut new_pos = robot.pos + total_vel;

        new_pos.x = new_pos.x.rem_euclid(ROWS);
        new_pos.y = new_pos.y.rem_euclid(COLS);

        positions.push(Vector2::new(new_pos.x as usize, new_pos.y as usize));
    }

    let grid = draw_grid(positions);

    println!("- {step} -------");
    print_grid(grid);
    println!()
}

pub fn main() {
    let s = read_input("src/day14/input.txt");

    let robots = parse_input(s);

    for i in 2..100 {
        let step = i*100 + (i - 2);
        calculate_step(&robots, step);
    }
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn test_case_one() {
        let s = read_input("src/day14/test.txt");

        let robots = parse_input(s);

        let mut positions: Vec<Vector2<usize>> = Vec::new();
        for robot in robots {
            let total_vel = robot.vel * 100;
            let mut new_pos = robot.pos + total_vel;

            new_pos.x = new_pos.x.rem_euclid(11);
            new_pos.y = new_pos.y.rem_euclid(7);

            positions.push(Vector2::new(new_pos.x as usize, new_pos.y as usize));
        }

        let grid = draw_grid(positions);
        print_grid(grid);
        let counts = count_grid(grid);

        let expected: [[usize; 2]; 2] = [[1, 3], [4, 1]];

        assert_eq!(counts, expected);
    }
}
