use std::{cell::RefCell, collections::VecDeque, rc::Rc, sync::OnceLock};

use crate::read_input;

#[allow(unused)]
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

#[derive(Debug)]
struct Block {
    contents: Vec<Option<usize>>,
    free: usize,
    start: usize,
    size: OnceLock<usize>,
    block_idx: usize,
}

type FileBlock = Rc<RefCell<Block>>;

impl Block {
    fn zero(&mut self) {
        self.contents = self.contents.iter().map(|_| None).collect();
        self.free = self.contents.len();
    }

    fn insert(&mut self, other: FileBlock) {
        let other_ref = other.borrow();
        let start_idx = self.size.get().unwrap() - self.free;

        self.contents.splice(
            start_idx..(start_idx + other_ref.size.get().unwrap()),
            other_ref.contents.clone(),
        );

        self.free -= other_ref.size.get().unwrap();
    }
}

fn parse_disk_map_chunked(s: String) -> (Vec<FileBlock>, VecDeque<FileBlock>) {
    let mut sector_count = 0usize;
    let mut global_idx = 0usize;

    let mut result: Vec<Rc<RefCell<Block>>> = Vec::new();
    let mut file_blocks: VecDeque<Rc<RefCell<Block>>> = VecDeque::new();

    for (idx, c) in s.chars().enumerate() {
        let n = c.to_digit(10).unwrap();

        if idx.rem_euclid(2) == 0 {
            let mut contents = Vec::new();
            let start = global_idx;

            for _ in 0..n {
                contents.push(Some(sector_count));
                global_idx += 1;
            }

            let size = OnceLock::new();
            size.set(n as usize).unwrap();

            let chunk = Block {
                size,
                contents,
                free: 0,
                start,
                block_idx: idx,
            };

            let rc_refcell = Rc::new(RefCell::new(chunk));

            file_blocks.push_front(rc_refcell.clone());
            result.push(rc_refcell.clone());

            sector_count += 1;
        } else {
            let mut contents: Vec<Option<usize>> = Vec::new();
            let start = global_idx;

            for _ in 0..n {
                contents.push(None);
                global_idx += 1;
            }

            let size = OnceLock::new();
            size.set(n as usize).unwrap();

            let chunk = Block {
                size,
                free: contents.len(),
                contents,
                start,
                block_idx: idx,
            };

            let rc_chunk = Rc::new(RefCell::new(chunk));

            result.push(rc_chunk);
        }
    }

    (result, file_blocks)
}

#[allow(unused)]
fn calculate_final_space(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map.iter().filter(|item| item.is_some()).count()
}

#[allow(unused)]
fn checksum(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, item)| idx * item.unwrap_or_default())
        .sum()
}

fn checksum_blocks(disk_map: &Vec<FileBlock>) -> usize {
    let mut sum = 0usize;

    for block in disk_map.iter() {
        let block_ref = block.borrow();

        for (idx, item) in block_ref.contents.iter().enumerate() {
            sum += item.unwrap_or_default() * (idx + block_ref.start);
        }
    }
    sum
}

pub fn day_9() {
    let s = read_input("src/day9/input.txt");

    let (disk_map, mut file_blocks) = parse_disk_map_chunked(s);

    'outer: loop {
        if let Some(block) = file_blocks.pop_front() {
            let block_idx = block.borrow().block_idx;

            if block_idx == 0 {
                break 'outer;
            }

            'inner: for idx in 0..block_idx - 1 {
                let item = disk_map[idx].clone();

                if item.borrow().free >= *block.borrow().size.get().unwrap() {
                    item.borrow_mut().insert(block.clone());
                    block.borrow_mut().zero();

                    break 'inner;
                }
            }
        } else {
            break 'outer;
        }
    }
    let sum = checksum_blocks(&disk_map);
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

    #[test]
    fn first_result() {
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
        assert_eq!(sum, 6201130364722);
    }

    #[test]
    fn second_case() {
        let s = read_input("src/day9/test.txt");
        let (disk_map, file_blocks) = parse_disk_map_chunked(s);

        dbg!(disk_map, file_blocks);

        // assert_eq!(sum, 2858);
    }

    #[test]
    fn second_case_checksum() {
        let s = read_input("src/day9/test.txt");

        let (disk_map, mut file_blocks) = parse_disk_map_chunked(s);

        'outer: loop {
            if let Some(block) = file_blocks.pop_front() {
                let block_idx = block.borrow().block_idx;

                if block_idx == 0 {
                    break 'outer;
                }

                'inner: for idx in 0..block_idx - 1 {
                    let item = disk_map[idx].clone();

                    if item.borrow().free >= *block.borrow().size.get().unwrap() {
                        item.borrow_mut().insert(block.clone());
                        block.borrow_mut().zero();

                        break 'inner;
                    }
                }
            } else {
                break 'outer;
            }
        }

        let sum = checksum_blocks(&disk_map);

        assert_eq!(sum, 2858);
    }
}
