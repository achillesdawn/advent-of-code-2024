fn read_input() -> String {
    let mut current_dir = std::env::current_dir().unwrap();

    current_dir.push("src/day2/input.txt");

    let file = std::fs::read_to_string(current_dir).expect("could not find input.txt");

    file
}

#[allow(unused)]
fn process_line(line: &str) -> bool {
    let split = line.split(" ");

    let split: Vec<i16> = split.map(|item| item.parse::<i16>().unwrap()).collect();

    let mut last = 0i16;
    let mut diff: i16;
    let incrementing = (split[1] - split[0]) > 0;

    for (idx, item) in split.into_iter().enumerate() {
        if idx == 0 {
            last = item;
            continue;
        }

        diff = item - last;

        let abs_diff = diff.abs();
        if ((diff > 0) == incrementing) && (1..=3).contains(&abs_diff) {
        } else {
            // dbg!(line, idx, item, last, incrementing, diff, abs_diff, diff > 0, (diff > 0) == incrementing, (1..=3).contains(&abs_diff));
            return false;
        }

        last = item;
    }

    true
}

#[allow(unused)]
pub fn problem1() {
    let s = read_input();

    let lines = s.lines();

    let result: usize = lines.map(process_line).filter(|i| *i).count();
    dbg!(result);
}

fn check_abs(item: i16, last: i16) -> bool {
    let diff = item - last;

    let abs_diff = diff.abs();
    (1..=3).contains(&abs_diff)
}

fn check(item: i16, last: i16, incrementing: bool) -> bool {
    let diff = item - last;

    let abs_diff = diff.abs();

    dbg!(
        diff,
        abs_diff,
        diff > 0,
        (diff > 0) == incrementing,
        (1..=3).contains(&abs_diff)
    );
    ((diff > 0) == incrementing) && (1..=3).contains(&abs_diff)
}

fn line_to_num(line: &str) -> Vec<i16> {
    let split = line.split(" ");

    split.map(|item| item.parse::<i16>().unwrap()).collect()
}

fn process_line_problem_2(split: Vec<i16>, missed: bool) -> bool {
    let mut last = 0i16;
    let incrementing = (split[1] - split[0]) > 0;

    for (idx, current) in split.iter().enumerate() {
        if idx == 0 {
            last = *current;
            continue;
        }

        let diff = current - last;

        let abs_diff = diff.abs();

        let safe = ((diff > 0) == incrementing) && (1..=3).contains(&abs_diff);

        if !safe {
            if !missed {
                let without_current: Vec<i16> = split
                    .iter()
                    .enumerate()
                    .filter(|(id, _)| *id != idx)
                    .map(|(_, i)| *i)
                    .collect();

                let without_current = process_line_problem_2(without_current, true);

                if without_current {
                    return true;
                }

                let without_last: Vec<i16> = split
                    .iter()
                    .enumerate()
                    .filter(|(id, _)| *id != (idx - 1))
                    .map(|(_, i)| *i)
                    .collect();

                let without_last = process_line_problem_2(without_last, true);

                if without_last {
                    return true;
                }

                if idx >= 2 {
                    let without_previous: Vec<i16> = split
                        .iter()
                        .enumerate()
                        .filter(|(id, _)| *id != (idx - 2))
                        .map(|(_, i)| *i)
                        .collect();

                    let without_previous = process_line_problem_2(without_previous, true);

                    if without_previous {
                        return true;
                    }
                }
            }

            return false;
        }

        last = *current;
    }

    true
}

pub fn problem2() {
    let s = read_input();

    let lines = s.lines();

    let result: usize = lines
        .map(line_to_num)
        .map(|i| process_line_problem_2(i, false))
        .filter(|i| *i)
        .count();

    dbg!(result);
}

#[cfg(test)]
mod test {
    use crate::day2::line_to_num;

    use super::{process_line, process_line_problem_2};

    // #[test]
    // fn test_case_one() {
    //     let s = "8 11 14 16 15";
    //     let safe = super::process_line(s);
    //     assert_eq!(safe, false);

    //     let s = "8 11 14 16 17";
    //     let safe = super::process_line(s);
    //     assert_eq!(safe, true);
    // }

    #[test]
    fn test_case_one_two() {
        let s = "8 8 14 16 15";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, false);

        let s = "8 1 14 16 17";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, false);

        let s = "8 9 14 16 17";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, false);

        let s = "8 11 14 16 17";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "8 7 8 16 17";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, false);
    }

    #[test]
    fn test_problem_two_one() {
        let s = "7 6 4 2 1";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "1 2 7 8 9";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, false);

        let s = "9 7 6 2 1";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, false);

        let s = "1 3 2 4 5";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "8 6 4 4 1";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "1 3 6 7 9";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "63 67 70 72 74 75 78 79";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "67 70 72 74 75 78 79 99";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "67 68 1 71 74 75 78 79";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "46 43 47 48 49 50";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);

        let s = "44 47 48 49 50 54";
        let safe = process_line_problem_2(line_to_num(s), false);
        assert_eq!(safe, true);
    }
}
