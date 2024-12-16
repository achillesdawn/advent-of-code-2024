use nalgebra::Vector2;

use crate::read_input;

#[derive(Debug)]
struct Robot {
    pos: Vector2<isize>,
    vel: Vector2<isize>,
}

fn parse_vector(s: &str) -> Vector2<isize> {
    let (x, y) = s.split_once("=").unwrap().1.split_once(",").unwrap();

    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

    Vector2::new(x, y)
}

fn parse_input(s: String) -> Vec<Robot> {
    s.lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" ").unwrap();

            let pos = parse_vector(pos);
            let vel = parse_vector(vel);

            Robot { pos, vel }
        })
        .collect()
}

pub fn main() {
    let s = read_input("src/day14/test.txt");

    let robots = parse_input(s);

    dbg!(robots);
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn test_case_one() {
        let s = read_input("src/day14/test.txt");

        let robots = parse_input(s);

        for robot in robots {
            let total_vel =robot.vel * 100;
            let new_pos = robot.pos + total_vel;

            let new_x = new_pos.x.rem_euclid(11);
            let new_y = new_pos.y.rem_euclid(7);

            dbg!(new_pos, new_x,new_y);
        }
    }
}
