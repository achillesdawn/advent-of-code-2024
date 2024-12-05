use std::{collections::HashSet, fmt::Display};

use super::{get_opposite_diagonal, Diagonal};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,

    UpRight,
    UpLeft,

    DownRight,
    DownLeft,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "UP"),
            Direction::Down => write!(f, "DOWN"),
            Direction::Left => write!(f, "LEFT"),
            Direction::Right => write!(f, "RIGHT"),
            Direction::UpRight => write!(f, "UP-RIGHT"),
            Direction::UpLeft => write!(f, "UP-LEFT"),
            Direction::DownRight => write!(f, "DOWN-RIGHT"),
            Direction::DownLeft => write!(f, "DOWN-LEFT"),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
    pub cols: usize,
    pub rows: usize,

    lookup: LookUp,
    pub diagonal_set: HashSet<Diagonal>,
}

#[allow(unused)]
impl Grid {
    pub fn new(s: String) -> Grid {
        let result: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let cols = result.len();
        let rows = result[0].len();

        let mut diagonal_set = HashSet::new();

        Grid {
            grid: result,
            rows,
            cols,
            lookup: LookUp::new(),
            diagonal_set,
        }
    }

    pub fn new_reversed(s: String) -> Grid {
        let result: Vec<Vec<char>> = s.lines().map(|line| line.chars().rev().collect()).collect();

        let cols = result.len();
        let rows = result[0].len();

        let mut diagonal_set = HashSet::new();

        Grid {
            grid: result,
            rows,
            cols,
            lookup: LookUp::new(),
            diagonal_set,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if (0..self.rows).contains(&x) && (0..self.cols).contains(&y) {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn x_plus(&self, x: usize) -> Option<usize> {
        if x >= self.rows {
            return None;
        }

        Some(x + 1)
    }

    fn y_plus(&self, y: usize) -> Option<usize> {
        if y >= self.cols {
            return None;
        }

        Some(y + 1)
    }

    fn x_y_minus(&self, y: usize) -> Option<usize> {
        y.checked_sub(1)
    }

    pub fn check_direction(&mut self, x: usize, y: usize, direction: &Direction) -> bool {
        // println!("checking x:{} y:{} direction: {}", x, y, direction);

        let (x, y) = match direction {
            Direction::Up => {
                let Some(y) = self.x_y_minus(y) else {
                    return false;
                };

                (x, y)
            }
            Direction::Down => {
                let Some(y) = self.y_plus(y) else {
                    return false;
                };

                (x, y)
            }
            Direction::Left => {
                let Some(x) = self.x_y_minus(x) else {
                    return false;
                };

                (x, y)
            }
            Direction::Right => {
                let Some(x) = self.x_plus(x) else {
                    return false;
                };

                (x, y)
            }
            Direction::UpRight => {
                let Some(y) = self.x_y_minus(y) else {
                    return false;
                };

                let Some(x) = self.x_plus(x) else {
                    return false;
                };

                (x, y)
            }
            Direction::UpLeft => {
                let Some(y) = self.x_y_minus(y) else {
                    return false;
                };

                let Some(x) = self.x_y_minus(x) else {
                    return false;
                };

                (x, y)
            }
            Direction::DownRight => {
                let Some(y) = self.y_plus(y) else {
                    return false;
                };

                let Some(x) = self.x_plus(x) else {
                    return false;
                };

                (x, y)
            }
            Direction::DownLeft => {
                let Some(y) = self.y_plus(y) else {
                    return false;
                };

                let Some(x) = self.x_y_minus(x) else {
                    return false;
                };

                (x, y)
            }
        };

        // dbg!((x, y));

        if let Some(c) = self.get(x, y) {
            if c == self.lookup.ch {
                let more = self.lookup.next_lookup();

                if !more {
                    return true;
                }

                if self.check_direction(x, y, direction) {
                    self.lookup.restore_lookup();
                    return true;
                }
            }
        }
        self.lookup.restore_lookup();
        false
    }

    pub fn check_diagonal(&mut self, diagonal: &Diagonal) -> bool {
        if let Some(next_c) = self.get(diagonal.x, diagonal.y) {
            if next_c != 'M' {
                return false;
            }

            let opposite_diagonal = get_opposite_diagonal(diagonal);

            if self.diagonal_set.contains(&diagonal)
                || self.diagonal_set.contains(&opposite_diagonal)
            {
                return false;
            }
            let checked_result = self.check_direction(diagonal.x, diagonal.y, &diagonal.direction);

            if !checked_result {
                return false;
            }

            self.diagonal_set.insert(diagonal.clone());
            self.diagonal_set.insert(opposite_diagonal);

            return true;
        }

        false
    }
}

#[derive(Debug)]
struct LookUp {
    ch: char,
}

impl LookUp {
    fn new() -> Self {
        LookUp { ch: 'A' }
    }

    fn restore_lookup(&mut self) {
        self.ch = 'A';
    }

    fn next_lookup(&mut self) -> bool {
        self.ch = match self.ch {
            'A' => 'S',
            _ => return false,
        };

        return true;
    }
}
