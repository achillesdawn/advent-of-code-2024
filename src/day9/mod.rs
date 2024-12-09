use std::collections::VecDeque;

use crate::read_input;

fn parse_disk_map(s: String) -> (Vec<Option<usize>>, VecDeque<usize>) {
    let mut sector_count = 0usize;
    let mut global_idx = 0usize;

    let mut result: Vec<Option<usize>> = Vec::new();
    let mut reverse_lookup = VecDeque::new();

    for (idx, c) in s.chars().enumerate() {
        let n = c.to_digit(10).unwrap();

        if idx.rem_euclid(2) == 0 {
            println!("sector {} files {}", sector_count, n);
            for _ in 0..n {
                result.push(Some(sector_count));
                reverse_lookup.push_front(global_idx);
                global_idx += 1;
            }
            sector_count += 1;
        } else {
            for _ in 0..n {
                result.push(None);
                global_idx += 1;
            }
            println!("{} spaces", n);
        }
    }
    (result, reverse_lookup)
}

fn calculate_final_space(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map.iter().filter(|item| item.is_some()).count()
}

fn checksum(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, item)| idx * item.unwrap_or_default())
        .sum()
}

pub fn day_9() {
    let s = read_input("src/day9/input.txt");
    let (mut disk_map, mut lookup) = parse_disk_map(s);
    let final_space = calculate_final_space(&disk_map);

    for idx in 0..disk_map.len() {
        let item = disk_map[idx];

        if item.is_none() {
            let swap_index = lookup.pop_front().unwrap();
            disk_map.swap(idx, swap_index);
        }

        if idx == final_space - 1 {
            break;
        }
    }

    let sum = checksum(&disk_map);
    dbg!(sum);
}

#[cfg(test)]
mod test {
    use crate::read_input;

    use super::*;

    #[test]
    fn test_case() {
        let s = read_input("src/day9/test.txt");

        dbg!(&s);
        let (disk_map, _) = parse_disk_map(s);

        let compare: Vec<Option<usize>> = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|c| c.to_digit(10).and_then(|x| Some(x as usize)))
            .collect();

        assert_eq!(disk_map, compare);
    }

    #[test]
    fn count_end_size() {
        let s = read_input("src/day9/test.txt");
        let (disk_map, _) = parse_disk_map(s);
        let space = calculate_final_space(&disk_map);

        assert_eq!(space, "0099811188827773336446555566".len())
    }

    #[test]
    fn first_case() {
        let s = read_input("src/day9/test.txt");
        let (mut disk_map, mut lookup) = parse_disk_map(s);
        let final_space = calculate_final_space(&disk_map);

        dbg!(&disk_map, &lookup);

        for idx in 0..disk_map.len() {
            let item = disk_map[idx];

            if item.is_none() {
                let swap_index = lookup.pop_front().unwrap();
                println!("swapping indices: {} with {}", idx, swap_index);
                disk_map.swap(idx, swap_index);
            }

            if idx == final_space - 1 {
                break;
            }
        }

        let compare_string: String = disk_map
            .iter()
            .map(|item| item.and_then(|n| Some(n.to_string().chars().next().unwrap())))
            .map(|item| item.or(Some('.')))
            .flatten()
            .collect();

        assert_eq!("0099811188827773336446555566..............", compare_string)
    }

    #[test]
    fn first_case_with_checksum() {
        let s = read_input("src/day9/test.txt");
        let (mut disk_map, mut lookup) = parse_disk_map(s);
        let final_space = calculate_final_space(&disk_map);

        for idx in 0..disk_map.len() {
            let item = disk_map[idx];

            if item.is_none() {
                let swap_index = lookup.pop_front().unwrap();

                disk_map.swap(idx, swap_index);
            }

            if idx == final_space - 1 {
                break;
            }
        }

        let sum = checksum(&disk_map);

        assert_eq!(sum, 1928);
    }
}
