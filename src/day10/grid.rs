use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
    path::PathBuf,
};

use super::vector::Vector;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Trail {
    start: Vector,
    current_height: u32,
    pos: Vector,
}

impl Trail {
    pub fn new(x: usize, y: usize) -> Self {
        Trail {
            start: Vector::new(x, y),
            pos: Vector::new(x, y),
            current_height: 0u32,
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
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub cols: usize,
    pub rows: usize,
}

impl Grid {
    pub fn new(s: String) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let cols = grid.len();
        let rows = grid[0].len();

        Grid { grid, cols, rows }
    }

    fn follow_trailhead(&self, trailhead: Trail) -> Vec<Trail> {
        if trailhead.current_height == 9 {
            return vec![trailhead];
        }

        let directions = trailhead.get_directions();

        let directions = directions.map(|item| {
            item.and_then(|v| {
                self.ascent(v.x, v.y).and_then(|n| {
                    if n == trailhead.current_height + 1 {
                        return Some(v);
                    }

                    None
                })
            })
        });

        let mut trails = Vec::new();

        if directions.iter().flatten().count() > 0 {
            for direction in directions {
                if let Some(direction) = direction {
                    dbg!(&direction);
                    let mut t = trailhead.clone();
                    t.pos = direction;
                    t.current_height += 1;
                    let followed_trails = self.follow_trailhead(t);
                    trails.extend_from_slice(&followed_trails);
                }
            }
        }

        trails
    }

    pub fn collect_trailheads(&mut self) -> HashSet<Trail> {
        let mut trails = HashSet::new();

        for col in 0..self.cols {
            for row in 0..self.rows {
                let c = self.grid[col][row];

                if c == '0' {
                    let trailhead = Trail::new(row, col);

                    let new_trails = self.follow_trailhead(trailhead);
                    
                    for trail in new_trails {
                        trails.insert(trail);
                    }
                }
            }
        }

        trails
    }

    pub fn collect_trailheads_ratings(&mut self) -> Vec<Trail> {
        let mut trails = Vec::new();

        for col in 0..self.cols {
            for row in 0..self.rows {
                let c = self.grid[col][row];

                if c == '0' {
                    let trailhead = Trail::new(row, col);

                    let new_trails = self.follow_trailhead(trailhead);
                    
                    trails.extend(new_trails);
                }
            }
        }

        trails
    }

    fn ascent(&self, x: usize, y: usize) -> Option<u32> {
        if self.check(x, y) {
            let c = self.grid[y][x];
            return c.to_digit(10);
        }

        None
    }

    fn check(&self, x: usize, y: usize) -> bool {
        if (0..self.rows).contains(&x) && (0..self.cols).contains(&y) {
            return true;
        }

        false
    }
}

#[allow(unused)]
impl Grid {
    pub fn print_grid(&self) {
        for (col, outer) in self.grid.iter().enumerate() {
            print!("{:<3} ", col);
            for c in outer.iter() {
                print!("{}", c);
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
