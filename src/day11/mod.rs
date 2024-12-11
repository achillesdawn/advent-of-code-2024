use crate::read_input;

fn parse_input(s: String) -> Vec<String> {
    let a: Vec<String> = s.split(" ").map(|i| i.to_owned()).collect();

    a
}

fn check_zeroes(s: &str) -> &str {
    if s.chars().filter(|c| *c == '0').count() == s.len() {
        "0"
    } else {
        s
    }
}

fn blink(a: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for idx in 0..a.len() {
        let item = &a[idx];

        match item.as_str() {
            "0" => {
                result.push("1".to_owned());
            }
            s if s.len().rem_euclid(2) == 0 => {
                let (first, last) = s.split_at(s.len() / 2);

                let first = first.parse::<u32>().unwrap().to_string();
                let last = last.parse::<u32>().unwrap().to_string();

                result.extend_from_slice(&[first, last]);
            }

            s => {
                let n = s.parse::<usize>().unwrap();
                let n = n * 2024;
                result.push(n.to_string());
            }
        }
    }

    result
}

pub fn day11() {
    let s = read_input("src/day11/input.txt");
    let mut a = parse_input(s);

    for _ in 0..75 {
        a = blink(a);
    }

    dbg!(a.len());
}

#[cfg(test)]
mod test {
    use crate::read_input;

    use super::*;

    fn vec_str_string(s: &[&str]) -> Vec<String> {
        s.into_iter().map(|i| (*i).to_owned()).collect()
    }

    #[test]
    fn test_case_one() {
        let s = read_input("src/day11/test.txt");
        let a = parse_input(s);
        let a = blink(a);

        let compare = vec_str_string(&["1", "2024", "1", "0", "9", "9", "2021976"]);

        assert_eq!(a, compare);
    }

    #[test]
    fn test_case_two() {
        let s = read_input("src/day11/example_2.txt");
        let a = parse_input(s);
        let a = blink(a);

        let compare = vec_str_string(&["253000", "1", "7"]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_str_string(&["253", "0", "2024", "14168"]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_str_string(&["512072", "1", "20", "24", "28676032"]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_str_string(&["512", "72", "2024", "2", "0", "2", "4", "2867", "6032"]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_str_string(&[
            "1036288", "7", "2", "20", "24", "4048", "1", "4048", "8096", "28", "67", "60", "32",
        ]);

        assert_eq!(a, compare);

        let a = blink(a);

        let compare = vec_str_string(&[
            "2097446912",
            "14168",
            "4048",
            "2",
            "0",
            "2",
            "4",
            "40",
            "48",
            "2024",
            "40",
            "48",
            "80",
            "96",
            "2",
            "8",
            "6",
            "7",
            "6",
            "0",
            "3",
            "2",
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
}
