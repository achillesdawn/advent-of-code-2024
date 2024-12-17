use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

use colored::Colorize;

use super::{direction::Direction, vector::Vector};

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub cols: usize,
    pub rows: usize,

    pos: Vector,
}

impl Grid {
    pub fn new(s: String) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let cols = grid.len();
        let rows = grid[0].len();

        let pos = Grid::find_start_pos(cols, rows, &grid);

        Grid {
            grid,
            cols,
            rows,
            pos,
        }
    }

    fn find_start_pos(cols: usize, rows: usize, grid: &Vec<Vec<char>>) -> Vector {
        for y in 0..cols {
            for x in 0..rows {
                let c = grid[y][x];

                if c == '@' {
                    return Vector::new(x, y);
                }
            }
        }
        panic!();
    }

    fn get(&self, next_pos: Option<Vector>) -> Option<(char, Vector)> {
        next_pos.and_then(|v| {
            if (0..self.rows).contains(&v.x) && (0..self.cols).contains(&v.y) {
                return Some((self.grid[v.y][v.x], v));
            } else {
                return None;
            }
        })
    }

    pub fn peak_next(&self, direction: &Direction) -> Option<(char, Vector)> {
        match direction {
            Direction::Up => self.get(self.pos.subtract(0, 1)),
            Direction::Down => self.get(Some(self.pos.add(0, 1))),
            Direction::Right => self.get(Some(self.pos.add(1, 0))),
            Direction::Left => self.get(self.pos.subtract(1, 0)),
        }
    }

    fn peak_next_at(&self, pos: &Vector, direction: &Direction) -> Option<(char, Vector)> {
        match direction {
            Direction::Up => self.get(pos.subtract(0, 1)),
            Direction::Down => self.get(Some(pos.add(0, 1))),
            Direction::Right => self.get(Some(pos.add(1, 0))),
            Direction::Left => self.get(pos.subtract(1, 0)),
        }
    }

    pub fn update_position(&mut self, pos: &Vector, new: &Vector) {
        let pos_c = self.grid[pos.y][pos.x];
        let new_c = self.grid[new.y][new.x];

        self.grid[pos.y][pos.x] = new_c;
        self.grid[new.y][new.x] = pos_c;
    }

    fn push(&mut self, position: &Vector, direction: &Direction) -> Option<Vector> {
        if let Some((c, next_pos)) = self.peak_next_at(&position, &direction) {
            println!(
                "Pushing at x:{}, y:{} towards {}",
                position.x, position.y, c
            );
            match c {
                '#' => return None,

                '.' => {
                    return Some(next_pos);
                }

                '[' => match &direction {
                    Direction::Up | Direction::Down => {
                        if let (Some(v1), Some(v2)) = (
                            self.push(&next_pos, &direction),
                            self.push(&&next_pos.add(1, 0), &direction),
                        ) {
                            self.update_position(&next_pos, &v1);
                            self.update_position(&next_pos.add(1, 0), &v2);
                            return Some(next_pos);
                        }

                        None
                    }

                    Direction::Right | Direction::Left => {
                        if let Some(v) = self.push(&next_pos, direction) {
                            self.update_position(&next_pos, &v);
                            return Some(next_pos);
                        }
                        None
                    }
                },

                ']' => match &direction {
                    Direction::Up | Direction::Down => {
                        if let (Some(v1), Some(v2)) = (
                            self.push(&next_pos, &direction),
                            self.push(&next_pos.subtract(1, 0).unwrap(), &direction),
                        ) {
                            self.update_position(&next_pos, &v1);
                            self.update_position(&next_pos.subtract(1, 0).unwrap(), &v2);
                            return Some(next_pos);
                        }

                        None
                    }

                    Direction::Right | Direction::Left => {
                        if let Some(v) = self.push(&next_pos, direction) {
                            self.update_position(&next_pos, &v);
                            return Some(next_pos);
                        }
                        None
                    }
                },

                _ => panic!(),
            }
        } else {
            return None;
        }
    }

    pub fn move_towards(&mut self, direction: Direction) {
        if let Some(vector) = self.push(&self.pos.clone(), &direction) {
            self.update_position(&self.pos.clone(), &vector);
            self.pos = vector;
        }
    }

    pub fn get_sum_coords(&self) -> usize {
        let mut sum_of_coords = 0usize;

        for y in 0..self.cols {
            for x in 0..self.rows {
                let c = self.grid[y][x];
                if c == 'O' {
                    sum_of_coords += (y * 100) + x;
                }
            }
        }

        sum_of_coords
    }
}

#[allow(unused)]
impl Grid {
    pub fn print_grid(&self) {
        for (col, outer) in self.grid.iter().enumerate() {
            print!("{:<3} ", col);
            for c in outer.iter() {
                match *c {
                    '@' => print!("{}", format!("{c}").red().bold()),
                    '[' => print!("{}", format!("{c}").blue().bold()),
                    ']' => print!("{}", format!("{c}").blue().bold()),
                    _ => print!("{c}"),
                }
            }

            print!("\n");
        }
    }

    pub fn write_to_file(&self, path: PathBuf) {
        let file = std::fs::File::create(path).unwrap();

        let mut buf = BufWriter::new(file);

        for (col, outer) in self.grid.iter().enumerate() {
            write!(buf, "{:<3} ", col).unwrap();
            ("{} ", col);
            for c in outer.iter() {
                write!(buf, "{}", c).unwrap();
            }

            write!(buf, "\n").unwrap();
        }
    }
}
