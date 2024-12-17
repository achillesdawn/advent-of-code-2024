use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
    path::PathBuf,
};

use colored::{Color, Colorize};

use super::{direction::Direction, vector::Vector};

#[derive(Debug, Clone)]
pub struct Plot {
    pub c: char,
    pos: Vector,
    members: HashMap<Vector, usize>,
}

impl Plot {
    pub fn new(pos: Vector, c: char) -> Self {
        Plot {
            members: HashMap::from([(pos.clone(), 0)]),
            pos,
            c,
        }
    }

    fn get_directions(&self) -> [Option<Vector>; 4] {
        [
            Some(self.pos.add(1, 0)),
            Some(self.pos.add(0, 1)),
            self.pos.subtract(1, 0),
            self.pos.subtract(0, 1),
        ]
    }

    pub fn calculate_area(&self) -> usize {
        self.members.len()
    }

    pub fn calculate_perimeter(&self) -> usize {
        let mut perimeter = 0usize;

        for value in self.members.values() {
            perimeter += 4 - value;
        }

        perimeter
    }
}

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

    fn push(&mut self, position: &Vector, direction: Direction) -> bool {
        if let Some((c, next_pos)) = self.peak_next_at(&position, &direction) {
            match c {
                '#' => return false,

                '.' => {
                    self.update_position(&position, &next_pos);
                    return true;
                }

                'O' => {
                    if self.push(&next_pos, direction) {
                        self.update_position(&position, &next_pos);
                        return true;
                    } else {
                        return false;
                    }
                }

                _ => panic!(),
            }
        } else {
            return false;
        }
    }

    pub fn move_towards(&mut self, direction: Direction) {
        if let Some((c, vector)) = self.peak_next(&direction) {
            match c {
                '.' => {
                    self.update_position(&self.pos.clone(), &vector);
                    self.pos = vector;
                }

                '#' => {}

                'O' => {
                    if self.push(&vector, direction) {
                        self.update_position(&self.pos.clone(), &vector);
                        self.pos = vector;
                    }
                }

                _ => panic!(),
            }
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
                    'O' => print!("{}", format!("{c}").blue().bold()),
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
