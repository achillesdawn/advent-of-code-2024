mod grid;
use std::collections::HashSet;

use grid::Grid;

use crate::read_input;

pub fn day_6_problem_one() {
    let s = read_input("src/day6/input.txt");
    let mut grid = Grid::new(s);

    grid.run();

    assert_eq!(grid.get_distinct_positions(), 5239);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    std::mem::swap(&mut grid.visited, &mut visited);

    let mut count = 0u32;

    for (x, y) in visited.iter() {
        if (*x == grid.start_pos.x) && (*y == grid.start_pos.y) {
            continue;
        }

        if *x== 88 && *y == 20 {
            println!("hello");
        }
        println!("outside putting obstacle x: {}, y:{}", *x, *y);

        grid.grid_put_obstacle(*x, *y);

        if grid.rerun_with_obstacle() {
            count += 1;
        }

        grid.grid_remove_obstacle();
    }

    dbg!(count);
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, str::FromStr};

    use grid::Grid;

    use crate::read_input;

    use super::*;

    #[test]
    fn test_start_pos() {
        let s = read_input("src/day6/test.txt");
        let grid = Grid::new(s);

        assert_eq!((grid.x, grid.y), (4, 6));
    }

    #[test]
    fn part_two() {
        let s = read_input("src/day6/test.txt");
        let mut grid = Grid::new(s);

        grid.run();

        assert_eq!(grid.get_distinct_positions(), 41);

        let visited = grid.get_visited().clone();

        let mut count = 0u32;

        let mut obstacles = Vec::new();

        for (x, y) in visited.iter() {
            if *x == grid.start_pos.x && *y == grid.start_pos.y {
                continue;
            }

            grid.grid_put_obstacle(*x, *y);

            if grid.rerun_with_obstacle() {
                obstacles.push((x, y));
                count += 1;
            }

            grid.grid_remove_obstacle();
        }

        dbg!(obstacles);

        assert_eq!(count, 6);
    }

    #[test]
    fn test_obstacle() {
        let s = read_input("src/day6/input.txt");
        let mut grid = Grid::new(s);

        grid.grid_put_obstacle(88, 20);
        grid.rerun_with_obstacle();
        grid.write_to_file(PathBuf::from_str("grid.txt").unwrap());
    }

    #[test]
    fn obstacle_put_and_remove() {
        let s = read_input("src/day6/input.txt");
        let mut grid = Grid::new(s);

        grid.grid_put_obstacle(88, 20);
        grid.grid_remove_obstacle();

        grid.write_to_file(PathBuf::from_str("obstacle.txt").unwrap());
    }
}
