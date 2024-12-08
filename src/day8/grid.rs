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

#[derive(Debug, PartialEq, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    fn new() -> Self {
        Vector { x: 0, y: 0 }
    }

    fn with_coords(x: i32, y: i32) -> Self {
        Vector { x, y }
    }

    fn subtract(&self, other: &Vector) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn add(&self, other: &Vector) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn invert(&self) -> Self {
        Vector {
            x: self.x * -1,
            y: self.y * -1,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub cols: usize,
    pub rows: usize,

    pub signals: HashMap<char, Vec<Signal>>,

    pub anti_nodes: HashSet<(i32, i32)>,
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
                                position: Vector::with_coords(row as i32, col as i32),
                            })
                        })
                        .or_insert(Vec::from([Signal {
                            position: Vector::with_coords(row as i32, col as i32),
                        }]));
                }
            }
        }
    }

    pub fn calculate_antinodes(&mut self) {
        for (signal_name, signals) in self.signals.iter() {
            for signal in signals.iter() {
                for other in signals.iter() {
                    if other.position != signal.position {
                        let separation = signal.position.subtract(&other.position);

                        let antinode_1 = signal.position.add(&separation);
                        let antinode_2 = other.position.add(&separation.invert());

                        if self.check(antinode_1.x, antinode_1.y) {
                            self.anti_nodes.insert((antinode_1.x, antinode_1.y));
                            self.grid[antinode_1.y as usize][antinode_1.x as usize] = '#'
                        }

                        if self.check(antinode_2.x, antinode_2.y) {
                            self.anti_nodes.insert((antinode_2.x, antinode_2.y));
                            self.grid[antinode_2.y as usize][antinode_2.x as usize] = '#'
                        }
                    }
                }
            }
        }
    }

    pub fn get_antinodes(&self) -> &HashSet<(i32, i32)> {
        &self.anti_nodes
    }

    pub fn get_distinct_positions(&self) -> usize {
        self.anti_nodes.len()
    }

    fn check(&self, x: i32, y: i32) -> bool {
        if (0..self.rows as i32).contains(&x) && (0..self.cols as i32).contains(&y) {
            return true;
        }

        false
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
