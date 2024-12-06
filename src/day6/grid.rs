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
}

impl Obstacle {
    fn new() -> Self {
        Obstacle {
            position: Vector::new(),
            hit_direction: None,
            num_hits: 0,
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

    pub start_pos: Vector,

    pub x: usize,
    pub y: usize,

    obstacle: Obstacle,

    visited: HashSet<(usize, usize)>,

    done: bool,
    is_loop: bool,
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

            start_pos: Vector::with_coords(pos.0, pos.1),

            obstacle: Obstacle::new(),

            visited: HashSet::new(),

            done: false,
            is_loop: false,
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

        // println!("moved: {},{}", self.x, self.y);
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
            } else if c == 'O' {
                

                if self.obstacle.num_hits == 0 {
                    self.obstacle.hit_direction = Some(self.direction.clone());
                    self.obstacle.num_hits += 1;
                } else {
                    
                    if let Some(obstacle_hit_direction) = &self.obstacle.hit_direction {
                        if self.direction == *obstacle_hit_direction {
                            self.done = true;
                            self.is_loop = true;
                            return;
                        } else {
                            self.obstacle.hit_direction = Some(self.direction.clone());
                        }
                    }

                    dbg!(self.obstacle.num_hits);
                }

                self.direction = self.direction.rotate();
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

    fn restart(&mut self) {
        self.x = self.start_pos.x;
        self.y = self.start_pos.y;

        self.done = false;
        self.is_loop = false;

        self.direction = Direction::Up;
    }

    pub fn rerun_with_obstacle(&mut self) -> bool {
        self.restart();

        while !self.done {
            self.peak_next();
        }

        if self.is_loop {
            return true;
        }

        false
    }

    pub fn grid_put_obstacle(&mut self, x: usize, y: usize) {
        self.obstacle.position = Vector::with_coords(x, y);
        self.obstacle.num_hits = 0;
        self.obstacle.hit_direction = None;

        self.grid[self.obstacle.position.y][self.obstacle.position.x] = 'O';

        // self.print_grid();
        // println!("putting obstacle at {},{}", x, y);
    }

    pub fn grid_remove_obstacle(&mut self) {
        self.grid[self.obstacle.position.y][self.obstacle.position.x] = '.';
    }

    pub fn get_visited(&self) -> &HashSet<(usize, usize)> {
        &self.visited
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
