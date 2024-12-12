use std::collections::{HashMap, VecDeque};

use crate::read_input;

fn parse_input(s: String) -> VecDeque<usize> {
    let a: VecDeque<usize> = s.split(" ").map(|i| i.parse::<usize>().unwrap()).collect();

    a
}

fn check_zeroes(s: &str) -> &str {
    if s.chars().filter(|c| *c == '0').count() == s.len() {
        "0"
    } else {
        s
    }
}

fn num_digits(mut n: usize) -> u32 {
    let mut count = 0u32;

    loop {
        count += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }

    count
}

fn blink(a: VecDeque<usize>) -> VecDeque<usize> {
    let mut result = VecDeque::new();

    for item in a.into_iter() {
        match item {
            0 => {
                result.push_back(1);
            }
            n => {
                let digit_count = num_digits(n);

                if digit_count.rem_euclid(2) == 0 {
                    let exp = 10usize.pow(digit_count / 2);

                    let first = n / exp;
                    let last = n.rem_euclid(first * exp);

                    result.push_back(first);
                    result.push_back(last);
                } else {
                    result.push_back(n * 2024);
                }
            }
        }
    }
    result
}

fn blink_zero_cached(a: VecDeque<usize>) -> (VecDeque<usize>, HashMap<usize, usize>) {
    let mut result = VecDeque::new();

    let mut number_count: HashMap<usize, usize> = HashMap::new();

    for item in a.into_iter() {
        match item {
            0 => {
                number_count.entry(0).and_modify(|e| *e += 1).or_insert(1);
            }
            n => {
                let digit_count = num_digits(n);

                if digit_count.rem_euclid(2) == 0 {
                    let exp = 10usize.pow(digit_count / 2);

                    let first = n / exp;
                    let last = n.rem_euclid(first * exp);

                    number_count
                        .entry(first)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    number_count
                        .entry(last)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                } else {
                    result.push_back(n * 2024);
                }
            }
        }
    }
    (result, number_count)
}

pub fn day11() {
    // let s = read_input("src/day11/input.txt");
    let mut a = parse_input("1950139".to_owned());

    let mut total_zeroes: HashMap<usize, usize> = HashMap::new();

    
}

#[cfg(test)]
mod test {
    use crate::read_input;

    use super::*;

    fn vec_usize(s: &[usize]) -> VecDeque<usize> {
        s.into_iter().map(|i| *i).collect()
    }

    #[test]
    fn test_case_one() {
        let s = read_input("src/day11/test.txt");
        let a = parse_input(s);
        let a = blink(a);

        let compare = vec_usize(&[1, 2024, 1, 0, 9, 9, 2021976]);

        assert_eq!(a, compare);
    }

    #[test]
    fn test_case_two() {
        let s = read_input("src/day11/example_2.txt");
        let a = parse_input(s);
        let a = blink(a);

        let compare = vec_usize(&[253000, 1, 7]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_usize(&[253, 0, 2024, 14168]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_usize(&[512072, 1, 20, 24, 28676032]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_usize(&[512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_usize(&[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_usize(&[
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2,
        ]);

        assert_eq!(a, compare);

        assert_eq!(a.len(), 22);
    }

    #[test]
    fn test_case_two_25_times() {
        let s = read_input("src/day11/example_2.txt");
        let mut a = parse_input(s);

        for _ in 0..25 {
            a = blink(a);
        }

        assert_eq!(a.len(), 55312);
    }

    #[test]
    fn test_num_digits() {
        let n = 233;

        let r = num_digits(n);

        assert_eq!(r, 3);

        let n = 2000;

        let r = num_digits(n);

        assert_eq!(r, 4);

        let n = 55555;

        let r = num_digits(n);

        assert_eq!(r, 5);

        let n = 7777777;

        let r = num_digits(n);

        assert_eq!(r, 7);

        let n = 88888888;

        let r = num_digits(n);

        assert_eq!(r, 8);
    }
}
