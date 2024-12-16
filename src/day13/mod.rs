use nalgebra::base::{Matrix2, Vector2};
use vector::Vector;

use crate::read_input;

mod vector;

const EPS: f64 = 0.000001f64;
const ONE_MINUS: f64 = 1.0 - EPS;

#[derive(Debug)]
struct Prize {
    a: Vector2<f64>,
    b: Vector2<f64>,
    prize: Vector2<f64>,
}

fn parse_input_vector(s: &str) -> Vector2<f64> {
    let (x, y) = s.split_once(",").unwrap();

    let x = x.split_once("+").unwrap().1.parse::<f64>().unwrap();
    let y = y.split_once("+").unwrap().1.parse::<f64>().unwrap();

    Vector2::new(x, y)
}

fn parse_prize_vector(s: &str) -> Vector2<f64> {
    let (x, y) = s.split_once(",").unwrap();

    let x = x.split_once("=").unwrap().1.parse::<f64>().unwrap();
    let y = y.split_once("=").unwrap().1.parse::<f64>().unwrap();

    Vector2::new(x, y)
}

fn parse_to_instructions(s: String) -> Vec<Prize> {
    let v: Vec<&str> = s.lines().collect();

    v.chunks(4)
        .map(|c| {
            let (a, b) = (
                c[0].split(":").last().unwrap(),
                c[1].split(":").last().unwrap(),
            );

            let (a, b) = (parse_input_vector(a), parse_input_vector(b));

            let prize = c[2].split(":").last().unwrap();
            let prize = parse_prize_vector(prize);

            Prize { a, b, prize }
        })
        .collect()
}

fn calculate(p: Prize) -> usize {
    let mat = Matrix2::new(p.a.x, p.b.x, p.a.y, p.b.y);

    if let Some(inv) = mat.try_inverse() {
        let solution = inv * p.prize;

        let x = solution[0];
        let y = solution[1];

        if (EPS..ONE_MINUS).contains(&x.fract()) || (EPS..ONE_MINUS).contains(&y.fract()) {
            println!("FAIL {x} {y}");
            0
        } else if x < 0.0 || y < 0.0 {
            println!("NEGATIVE {x} {y}");
            0
        } else {
            println!("SUCCESS {x} {y}");
            let x = x.round() as usize;
            let y = y.round() as usize;

            println!("ROUNDED {x} {y}");

            (x * 3) + y
        }
    } else {
        0
    }
}

pub fn day13() {
    let s = read_input("src/day13/input.txt");

    let prizes = parse_to_instructions(s);
    let mut cost = 0usize;
    for p in prizes {
        cost += calculate(p);
    }

    dbg!(cost);
}

#[cfg(test)]
mod test {

    use crate::read_input;

    use super::*;

    #[test]
    fn test_parse_case_one() {
        let s = read_input("src/day13/case_one.txt");

        let prizes = parse_to_instructions(s);

        dbg!(prizes);
    }

    #[test]
    fn test_case_one() {
        let s = read_input("src/day13/case_one.txt");

        let prizes = parse_to_instructions(s);

        let mut cost = 0usize;

        for p in prizes {
            cost += calculate(p);
        }

        assert_eq!(cost, 480);
    }
}
