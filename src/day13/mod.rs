use nalgebra::base::{Matrix2, Vector2};
use vector::Vector;

use crate::read_input;

mod vector;

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
pub fn day13() {
    let s = read_input("src/day13/case_one.txt");

    let prizes = parse_to_instructions(s);

    for p in prizes {
        let mat = Matrix2::new(p.a.x, p.b.x, p.a.y, p.b.y);

        if let Some(inv) = mat.try_inverse() {
            let solution = inv * p.prize;

            dbg!(solution);
        }
    }
}

#[cfg(test)]
mod test {
    use std::f64::EPSILON;

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

        const EPS: f64 = 0.000000000001f64;
        let s = read_input("src/day13/case_one.txt");

        let prizes = parse_to_instructions(s);

        for p in prizes {
            let mat = Matrix2::new(p.a.x, p.b.x, p.a.y, p.b.y);

            if let Some(inv) = mat.try_inverse() {
                let solution = inv * p.prize;

                let x = solution[0];
                let y = solution[1];

                if x.fract() > EPS || y.fract() > EPS {
                    println!("NO SOLUTION {x} {y}");
                } else {
                    println!("SOLUTION {x} {y}");
                }
            }
        }
    }
}
