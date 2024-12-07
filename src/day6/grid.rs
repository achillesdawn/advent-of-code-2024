use std::{collections::HashSet, fmt::Display};

mod direction;
use direction::Direction;

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
struct Obstacle {
    position: Vector,
    hit_direction: Option<Direction>,
    num_hits: u16,
    hits: HashSet<(usize, usize, Direction)>,
    repeated_hits: Vec<(usize, usize, Direction)>,
    last_hits: Vec<(usize, usize, Direction)>,
}

impl Obstacle {
    fn new() -> Self {
        Obstacle {
            position: Vector::new(),
            hit_direction: None,
            num_hits: 0,
            hits: HashSet::new(),
            repeated_hits: Vec::new(),
            last_hits: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.hit_direction = None;
        self.num_hits = 0;
        self.hits = HashSet::new();
        self.repeated_hits = Vec::new();
        self.last_hits = Vec::new();
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,

    pub cols: usize,
    pub rows: usize,

    pub direction: Direction,

    pub start_pos: Vector,

    pub x: usize,
    pub y: usize,

    obstacle: Obstacle,

    visited: HashSet<(usize, usize)>,

    num_moves: u32,

    done: bool,
    is_loop: bool,
    store_pos: bool,
}

impl Grid {
    pub fn new(s: String) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let cols = grid.len();
        let rows = grid[0].len();

        let pos = Grid::find_starting_pos(&grid);
        let start_pos = Vector::with_coords(pos.0, pos.1);

        Grid {
            grid,

            cols,
            rows,

            direction: Direction::Up,

            x: pos.0,
            y: pos.1,

            start_pos,

            obstacle: Obstacle::new(),

            visited: HashSet::new(),

            num_moves: 0,

            done: false,
            is_loop: false,
            store_pos: true,
        }
    }

    pub fn get_visited(&self) -> &HashSet<(usize, usize)> {
        &self.visited
    }

    pub fn get_distinct_positions(&self) -> usize {
        self.visited.len()
    }

    fn update_visited(&mut self) {
        // println!("moved: {},{}", self.x, self.y);
        if self.store_pos {
            self.visited.insert((self.x, self.y));
        } else {
            self.num_moves += 1;

            if self.num_moves > 10000 {
                self.done = true;
                self.is_loop = true;
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if (0..self.rows).contains(&x) && (0..self.cols).contains(&y) {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn make_a_move(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }

        self.update_visited();
    }

    fn update_hits(&mut self, x: usize, y: usize) -> bool {
        if self.obstacle.hits.contains(&(x, y, self.direction.clone())) {
            return true;
        } else {
            self.obstacle.hits.insert((x, y, self.direction.clone()));
            return false;
        }
    }

    fn peak_and_move(&mut self, x: usize, y: usize) {
        if let Some(c) = self.get(x, y) {
            if c == '#' {
                if self.update_hits(x, y) {
                    println!(
                        "repeated hit from same direction: {},{} {}",
                        x, y, self.direction
                    );

                    // self.obstacle.repeated_hits.push(value);
                }

                self.direction = self.direction.rotate();
            } else if c == 'O' {
                if self.update_hits(x, y) {
                    println!(
                        "repeated hit from same direction: {},{} {}",
                        x, y, self.direction
                    );
                    self.obstacle
                        .repeated_hits
                        .push((x, y, self.direction.clone()));
                }

                if self.obstacle.num_hits == 0 {
                    println!("obstacle first hit: direction: {}", self.direction);

                    self.obstacle.hit_direction = Some(self.direction.clone());
                    self.obstacle.num_hits += 1;
                } else {
                    self.obstacle.num_hits += 1;

                    println!("obstacle hits {}", self.obstacle.num_hits);
                    dbg!(&self.obstacle.repeated_hits);

                    if self.obstacle.repeated_hits == self.obstacle.last_hits {
                        self.done = true;
                        self.is_loop = true;
                        return;
                    } else {
                        self.obstacle.last_hits = self.obstacle.repeated_hits.clone();
                        self.obstacle.repeated_hits = Vec::new();
                    }
                }

                self.direction = self.direction.rotate();
            }

            self.make_a_move();
        } else {
            dbg!(self.x, self.rows, self.y, self.cols, &self.direction);
            println!("upper bounds exit");

            self.done = true;
            self.is_loop = false;
        }
    }

    fn peak_next(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.y.checked_sub(1).is_none() {
                    self.done = true;
                    self.is_loop = false;
                    println!("lower bounds exit");
                    return;
                }

                self.peak_and_move(self.x, self.y - 1)
            }
            Direction::Down => self.peak_and_move(self.x, self.y + 1),
            Direction::Right => self.peak_and_move(self.x + 1, self.y),
            Direction::Left => {
                if self.x.checked_sub(1).is_none() {
                    self.done = true;
                    self.is_loop = false;

                    dbg!(self.x, self.rows, self.y, self.cols, &self.direction);
                    println!("lower bounds exit");
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

    fn prepare_run(&mut self) {
        self.x = self.start_pos.x;
        self.y = self.start_pos.y;

        self.direction = Direction::Up;
        self.num_moves = 0;

        self.done = false;
        self.is_loop = false;

        self.store_pos = false;

        self.obstacle.reset();
    }

    pub fn rerun_with_obstacle(&mut self) -> bool {
        self.prepare_run();

        while !self.done {
            self.peak_next();
        }

        if self.is_loop {
            println!("is loop");
            return true;
        }

        false
    }

    pub fn grid_put_obstacle(&mut self, x: usize, y: usize) {
        self.obstacle.position = Vector::with_coords(x, y);

        self.grid[self.obstacle.position.y][self.obstacle.position.x] = 'O';

        // self.print_grid();
        println!("----------------");
        println!("putting obstacle at {},{}", x, y);
    }

    pub fn grid_remove_obstacle(&mut self) {
        self.grid[self.obstacle.position.y][self.obstacle.position.x] = '.';
    }
}

impl Grid {
    fn print_grid(&self) {
        for (col, outer) in self.grid.iter().enumerate() {
            print!("{} ", col);
            for c in outer.iter() {
                print!("{}", c);
            }

            print!("\n");
        }
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
