use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Signal {
    position: Vector,
}

#[derive(Debug)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

impl Vector {
    fn new() -> Self {
        Vector { x: 0, y: 0 }
    }

    fn with_coords(x: usize, y: usize) -> Self {
        Vector { x, y }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub cols: usize,
    pub rows: usize,

    pub signals: HashMap<char, Vec<Signal>>,

    pub anti_nodes: HashSet<(usize, usize)>,
}

impl Grid {
    pub fn new(s: String) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let cols = grid.len();
        let rows = grid[0].len();

        Grid {
            grid,

            cols,
            rows,

            signals: HashMap::new(),
            anti_nodes: HashSet::new(),
        }
    }

    pub fn collect_signals(&mut self) {
        for col in 0..self.cols {
            for row in 0..self.rows {
                let c = self.grid[col][row];

                if c != '.' {
                    let entry = self.signals.entry(c);
                    entry
                        .and_modify(|e| {
                            e.push(Signal {
                                position: Vector::with_coords(row, col),
                            })
                        })
                        .or_insert(Vec::from([Signal {
                            position: Vector::with_coords(row, col),
                        }]));
                }
            }
        }
    }

    pub fn get_antinodes(&self) -> &HashSet<(usize, usize)> {
        &self.anti_nodes
    }

    pub fn get_distinct_positions(&self) -> usize {
        self.anti_nodes.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if (0..self.rows).contains(&x) && (0..self.cols).contains(&y) {
            Some(self.grid[y][x])
        } else {
            None
        }
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
