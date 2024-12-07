use crate::read_input;

#[derive(Debug)]
struct Operator {
    result: u64,
    items: Vec<u64>,
}

fn parse_input(s: String) -> Vec<Operator> {
    s.lines()
        .map(|line| {
            let (result, items) = line.split_once(':').unwrap();

            let result: u64 = result.parse().unwrap();
            let items: Vec<u64> = items
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect();

            Operator { result, items }
        })
        .collect()
}

fn operate(operator: Operator) -> u64 {
    let mut sums: Vec<u64> = Vec::from([operator.items[0]]);
    let mut new_sums: Vec<u64> = Vec::new();

    for num in &operator.items[1..] {
        new_sums = Vec::new();

        for sum in sums {
            let mul = num * sum;
            let add = num + sum;

            if mul <= operator.result {
                new_sums.push(mul);
            }

            if add <= operator.result {
                new_sums.push(add);
            }
        }

        sums = new_sums;
    }

    if sums.contains(&operator.result) {
        // println!("{}: {:?} is OK", operator.result, operator.items);
        operator.result
    } else {
        // println!("{}: {:?} FAILS", operator.result, operator.items);
        0
    }
}

fn operate_with_concat(operator: Operator) -> u64 {
    let mut sums: Vec<u64> = Vec::from([operator.items[0]]);
    let mut new_sums: Vec<u64> = Vec::new();

    for num in &operator.items[1..] {
        new_sums = Vec::new();

        for sum in sums {
            let mul = num * sum;
            let add = num + sum;
            let concat = format!("{}{}", sum, num).parse::<u64>().unwrap();

            if mul <= operator.result {
                new_sums.push(mul);
            }

            if add <= operator.result {
                new_sums.push(add);
            }

            if concat <= operator.result {
                new_sums.push(concat);
            }
        }

        sums = new_sums;
    }

    if sums.contains(&operator.result) {
        // println!("{}: {:?} is OK", operator.result, operator.items);
        operator.result
    } else {
        // println!("{}: {:?} FAILS", operator.result, operator.items);
        0
    }
}

fn run(operators: Vec<Operator>) -> u64 {
    let mut sum = 0u64;

    for operator in operators.into_iter() {
        sum += operate(operator);
    }

    sum
}

fn run_part_two(operators: Vec<Operator>) -> u64 {
    let mut sum = 0u64;

    for operator in operators.into_iter() {
        sum += operate_with_concat(operator);
    }

    sum
}

pub fn day7_problem_one() {
    let s = read_input("src/day7/input.txt");
    let operators = parse_input(s);

    let result = run(operators);
    dbg!(result);
}

pub fn day7_problem_two() {
    let s = read_input("src/day7/input.txt");
    let operators = parse_input(s);

    let result = run_part_two(operators);
    dbg!(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let s = read_input("src/day7/test.txt");
        let operators = parse_input(s);

        let mut sum = 0u64;

        for operator in operators.into_iter() {
            sum += operate(operator);
        }

        assert_eq!(sum, 3749);
    }

    #[test]
    fn test_part_two() {
        let s = read_input("src/day7/test.txt");
        let operators = parse_input(s);

        let mut sum = 0u64;

        for operator in operators.into_iter() {
            sum += operate_with_concat(operator);
        }

        assert_eq!(sum, 11387);
    }
}
