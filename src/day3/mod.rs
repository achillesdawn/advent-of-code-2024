fn read_input() -> String {
    let mut current_dir = std::env::current_dir().unwrap();

    current_dir.push("src/day3/input.txt");

    let s = std::fs::read_to_string(current_dir).expect("could not find input.txt");

    s
}

#[allow(unused)]
fn parse_str(s: &str) -> u32 {
    let pattern = regex::Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();

    let caputures = pattern.captures_iter(s);

    let mut sum = 0u32;

    for capture in caputures {
        let x = capture.name("x");
        let y = capture.name("y");

        if let (Some(x), Some(y)) = (x, y) {
            let x = x.as_str().parse::<u32>().unwrap();
            let y = y.as_str().parse::<u32>().unwrap();

            sum += x * y;
        }
    }
    sum
}

fn parse_str_two(s: &str) -> u32 {
    let pattern =
        regex::Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let caputures = pattern.captures_iter(s);

    let mut sum = 0u32;
    let mut enabled = true;

    for capture in caputures {
        let x = capture.name("x");
        let y = capture.name("y");

        if let (Some(x), Some(y)) = (x, y) {
            let x = x.as_str().parse::<u32>().unwrap();
            let y = y.as_str().parse::<u32>().unwrap();

            if enabled {
                sum += x * y;
            }
        } else {
            let d = capture.get(0).unwrap();
            let d = d.as_str();
            if d == "don't()" {
                enabled = false;
            } else if d == "do()" {
                enabled = true;
            }
        }
    }
    sum
}

pub fn day_3_problem_one() {
    let s = read_input();
    let result = parse_str_two(&s);
    dbg!(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let r = parse_str(&input);
        assert_eq!(r, 161);
    }

    #[test]
    fn test_two() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let r = parse_str_two(s);
        assert_eq!(r, 48);
    }
}
