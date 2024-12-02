fn read_input() -> String {
    let mut current_dir = std::env::current_dir().unwrap();

    current_dir.push("src/day2/input.txt");

    let file = std::fs::read_to_string(current_dir).expect("could not find input.txt");

    file
}

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

fn check_abs(item: i16, last: i16) -> bool {
    let diff = item - last;

    let abs_diff = diff.abs();
    (1..=3).contains(&abs_diff)
}

fn check(item: i16, last: i16, incrementing: bool) -> bool {
    let diff = item - last;

    let abs_diff = diff.abs();

    // dbg!(diff, abs_diff, diff > 0, (diff > 0) == incrementing, (1..=3).contains(&abs_diff));
    ((diff > 0) == incrementing) && (1..=3).contains(&abs_diff)
}

fn process_line_problem_2(line: &str, missed: bool) -> bool {
    let split = line.split(" ");

    let split: Vec<i16> = split.map(|item| item.parse::<i16>().unwrap()).collect();

    let mut last = 0i16;
    let mut incrementing = (split[1] - split[0]) > 0;

    let mut missed = false;

    for (idx, current) in split.iter().enumerate() {
        if idx == 0 {
            last = *current;
            continue;
        }

        let safe = check(*current, last, incrementing);

        if safe {
        } else {
            dbg!(line, idx, current, last, incrementing);

            if !missed {
                missed = true;

                if idx == 1 {
                    let next = split[idx + 1];

                    let last_safe = check_abs(next, last);

                    let current_safe = check_abs(next, *current);

                    if last_safe {
                        println!("removing idx {} item {}", idx, current);
                        println!("-----------");
                        incrementing = (next - last) > 0;
                        continue;
                    } else if current_safe {
                        println!("removing idx {} item {}", idx - 1, last);
                        println!("-----------");
                        last = *current;

                        incrementing = (next - current) > 0;
                        continue;
                    } else {
                        println!("None safe");
                        println!("-----------");
                        return false;
                    }
                } else if idx == 2 {
                    let previous = split[idx - 2];

                    let last_safe = check_abs(previous, last);
                    let previous_safe = check_abs(previous, *current);

                    if last_safe {
                        println!("removing idx {} item {}", idx-1, last);
                        println!("-----------");
                        incrementing = (next - last) > 0;
                        continue;
                    } else if previous_safe {
                        println!("removing idx {} item {}", idx - 1, last);
                        println!("-----------");
                        last = *current;

                        incrementing = (next - current) > 0;
                        continue;
                    } else {
                        println!("None safe");
                        println!("-----------");
                        return false;
                    }
                } else {

                }

                continue;
            }
            println!("FAIL");
            println!("-----------");
            return false;
        }

        last = *current;
    }
    if missed {
        println!("PASS");
        println!("-----------");
    }

    true
}

pub fn problem1() {
    let s = read_input();

    let lines = s.lines();

    let result: usize = lines.map(process_line).filter(|i| *i).count();
    dbg!(result);
}

pub fn problem2() {
    let s = read_input();

    let lines = s.lines();

    let result: usize = lines.map(process_line_problem_2).filter(|i| *i).count();
    dbg!(result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_case_one() {
        let s = "8 11 14 16 15";
        let safe = super::process_line(s);
        assert_eq!(safe, false);

        let s = "8 11 14 16 17";
        let safe = super::process_line(s);
        assert_eq!(safe, true);
    }

    #[test]
    fn test_case_one_two() {
        let s = "8 8 14 16 15";
        let safe = super::process_line(s);
        assert_eq!(safe, false);

        let s = "8 1 14 16 17";
        let safe = super::process_line(s);
        assert_eq!(safe, false);

        let s = "8 9 14 16 17";
        let safe = super::process_line(s);
        assert_eq!(safe, false);

        let s = "8 11 14 16 17";
        let safe = super::process_line(s);
        assert_eq!(safe, true);

        let s = "8 7 8 16 17";
        let safe = super::process_line(s);
        assert_eq!(safe, false);
    }

    #[test]
    fn test_problem_two_one() {
        let s = "7 6 4 2 1";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "1 2 7 8 9";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, false);

        let s = "9 7 6 2 1";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, false);

        let s = "1 3 2 4 5";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "8 6 4 4 1";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "1 3 6 7 9";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "63 67 70 72 74 75 78 79";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "67 70 72 74 75 78 79 99";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "67 68 1 71 74 75 78 79";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);

        let s = "46 43 47 48 49 50";
        let safe = super::process_line_problem_2(s, false);
        assert_eq!(safe, true);
    }
}
