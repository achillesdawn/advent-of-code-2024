use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
    path::PathBuf,
};

use super::vector::Vector;

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
            perimeter += 4-value;
        }

        perimeter
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

    pub fn get_unique(&self) -> HashSet<char> {
        let mut unique = HashSet::new();

        for col in 0..self.cols {
            for row in 0..self.rows {
                let c = self.grid[col][row];
                unique.insert(c);
            }
        }

        unique
    }

    fn follow_trailhead(&self, mut plot: Plot) -> Plot {
        let directions = plot.get_directions();

        let directions = directions.map(|item| {
            item.and_then(|v| {
                self.ascent(v.x, v.y).and_then(|n| {
                    if n == plot.c {
                        return Some(v);
                    }

                    None
                })
            })
        });

        if directions.iter().flatten().count() > 0 {
            for direction in directions {
                if let Some(direction) = direction {
                    if plot.members.contains_key(&direction) {
                        let entry = plot.members.entry(direction);
                        entry.and_modify(|e| *e += 1).or_insert(1);
                        continue;
                    }

                    plot.members.insert(direction.clone(), 1);
                    plot.pos = direction;

                    plot = self.follow_trailhead(plot);
                }
            }

            return plot;
        } else {
            return plot;
        }
    }

    pub fn collect_plots(&mut self) -> Vec<Plot> {
        let mut plots = Vec::new();
        let mut accounted: HashSet<Vector> = HashSet::new();

        for col in 0..self.cols {
            for row in 0..self.rows {
                let pos = Vector::new(row, col);

                if accounted.contains(&pos) {
                    continue;
                }

                let c = self.grid[col][row];

                let plot = Plot::new(pos, c);

                let plot = self.follow_trailhead(plot);

                let keys: Vec<Vector> = plot.members.keys().map(|i| i.clone()).collect();

                accounted.extend(keys);
                plots.push(plot);
            }
        }

        plots
    }

    fn ascent(&self, x: usize, y: usize) -> Option<char> {
        if self.check(x, y) {
            let c = self.grid[y][x];
            return Some(c);
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
