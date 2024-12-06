use std::{collections::HashSet, fmt::Display};

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "UP"),
            Direction::Down => write!(f, "DOWN"),
            Direction::Left => write!(f, "LEFT"),
            Direction::Right => write!(f, "RIGHT"),
        }
    }
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub cols: usize,
    pub rows: usize,

    pub direction: Direction,

    pub x: usize,
    pub y: usize,

    visited: HashSet<(usize, usize)>,

    done: bool,
}

#[allow(unused)]
impl Grid {
    pub fn new(s: String) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let cols = grid.len();
        let rows = grid[0].len();

        let pos = Grid::find_starting_pos(&grid);

        Grid {
            grid,
            cols,
            rows,
            direction: Direction::Up,
            x: pos.0,
            y: pos.1,
            visited: HashSet::new(),
            done: false,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if (0..self.rows).contains(&x) && (0..self.cols).contains(&y) {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn update_visited(&mut self) {
        self.visited.insert((self.x, self.y));
    }

    fn make_a_move(&mut self, x: usize, y: usize) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }

        self.update_visited();
    }

    fn peak_and_move(&mut self, x: usize, y: usize) {
        if let Some(c) = self.get(x, y) {
            if c == '#' {
                // println!("found obstacle, rotating");

                self.direction = self.direction.rotate();
                // dbg!(&self.direction);
            }

            self.make_a_move(x, y);
        } else {
            self.done = true
        }
    }

    fn peak_next(&mut self) {
        match self.direction {
            Direction::Up => {
                let y = self.y.checked_sub(1);
                if y.is_none() {
                    self.done = true;
                    return;
                }
                self.peak_and_move(self.x, self.y - 1)
            }
            Direction::Down => self.peak_and_move(self.x, self.y + 1),
            Direction::Right => self.peak_and_move(self.x + 1, self.y),
            Direction::Left => {
                let x = self.x.checked_sub(1);
                if x.is_none() {
                    self.done = true;
                    return;
                }
                self.peak_and_move(self.x - 1, self.y)
            }
        }
    }

    pub fn run(&mut self) {
        // first position
        self.update_visited();

        while !self.done {
            self.peak_next();
            // dbg!((self.x, self.y));
        }
    }

    pub fn get_distinct_positions(&self) -> usize {
        self.visited.len()
    }

    fn find_starting_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
        for (col, outer) in grid.iter().enumerate() {
            for (row, c) in outer.iter().enumerate() {
                if *c == '^' {
                    return (row, col);
                }
            }
        }

        panic!()
    }
}
