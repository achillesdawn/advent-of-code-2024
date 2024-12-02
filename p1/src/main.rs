use std::collections::HashMap;

fn read_input() -> String {
    let file = std::fs::read_to_string("input.txt").expect("could not find input.txt");

    file
}

fn parse_lines(s: String) -> (Vec<i32>, Vec<i32>) {
    let mut list_a = Vec::with_capacity(1000);
    let mut list_b = Vec::with_capacity(1000);

    s.lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            (a, b)
        })
        .for_each(|(a, b)| {
            list_a.push(a);
            list_b.push(b);
        });

    (list_a, list_b)
}

fn problem1() {
    let s = read_input();

    let (mut list_a, mut list_b) = parse_lines(s);

    list_a.sort();
    list_b.sort();

    let mut result: u32 = 0;

    for (a, b) in list_a.into_iter().zip(list_b) {
        result += (a - b).abs() as u32;
    }

    dbg!(result);
}

fn create_sum_map(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut m = HashMap::new();

    for i in v.into_iter() {
        let entry = m.entry(i);
        entry.and_modify(|e| *e += 1).or_insert(1);
    }

    m
}

fn main() {
    let s = read_input();
    let (mut list_a, mut list_b) = parse_lines(s);

    let m = create_sum_map(list_b);

    let mut result = 0;
    for i in list_a.into_iter() {
        if let Some(num_times) = m.get(&i) {
            result += num_times * i;
        }
    }

    dbg!(result);
}

#[cfg(test)]
mod test {
    use crate::create_sum_map;

    #[test]
    fn test_map() {
        let test_case = Vec::from([1, 1, 1, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6]);

        let m = create_sum_map(test_case);

        assert_eq!(m[&1], 3);
        assert_eq!(m[&3], 3);
        assert_eq!(m[&4], 4);
        assert_eq!(m[&5], 5);
        assert_eq!(m[&6], 1);
    }
}
