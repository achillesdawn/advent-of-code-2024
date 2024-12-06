use std::collections::{HashMap, HashSet};

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).expect(&format!("could not find path: {}", path))
}

fn parse_input(s: String) -> (Vec<String>, Vec<String>) {
    let mut instructions = Vec::new();
    let mut updates = Vec::new();

    let mut instruction = true;
    for line in s.lines() {
        if line == "" {
            instruction = false;
            continue;
        }

        if instruction {
            instructions.push(line.into());
        } else {
            updates.push(line.into());
        }
    }

    (instructions, updates)
}

#[derive(Debug)]
struct Instruction {
    less: HashSet<String>,
    more: HashSet<String>,
}

impl Instruction {
    fn new() -> Self {
        let less = HashSet::new();
        let more = HashSet::new();

        Instruction { less, more }
    }
}

fn evaluate(instructions: &HashMap<String, Instruction>, updates: &Vec<String>) -> bool {
    let updates: HashMap<&String, usize> = updates
        .iter()
        .enumerate()
        .map(|(idx, i)| (i, idx))
        .collect();

    for (key, instruction) in instructions.iter() {
        if let Some(idx) = updates.get(key) {
            for less in instruction.less.iter() {
                if let Some(compare) = updates.get(less) {
                    if idx > compare {
                        return false;
                    }
                }
            }

            for more in instruction.more.iter() {
                if let Some(compare) = updates.get(more) {
                    if idx < compare {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn parse_instructions(instructions: Vec<String>) -> HashMap<String, Instruction> {
    let mut result: HashMap<String, Instruction> = HashMap::new();

    for instruction in instructions {
        let (first, last) = instruction.split_once("|").unwrap();

        let entry = result.entry(first.to_string());
        entry
            .and_modify(|e| {
                e.less.insert(last.to_owned());
            })
            .or_insert({
                let mut new_instruction = Instruction::new();
                new_instruction.less.insert(last.to_owned());
                new_instruction
            });

        let entry = result.entry(last.to_string());
        entry
            .and_modify(|e| {
                e.more.insert(first.to_owned());
            })
            .or_insert({
                let mut new_instruction = Instruction::new();
                new_instruction.more.insert(first.to_owned());
                new_instruction
            });
    }

    result
}

fn parse_updates(updates: Vec<String>) -> Vec<Vec<String>> {
    updates
        .into_iter()
        .map(|item| item.split(",").map(|i| i.to_owned()).collect())
        .collect()
}

fn order(instructions: &HashMap<String, Instruction>, updates: &Vec<String>) {
    dbg!(updates);
}

pub fn day5_first() {
    let s = read_input("src/day5/input.txt");
    let (instructions, updates) = parse_input(s);

    let instructions = parse_instructions(instructions);
    let updates = parse_updates(updates);

    let mut sum_middle_index = 0u32;

    for update in updates.iter() {
        if evaluate(&instructions, update) {
            // dbg!(update);
            let middle_index = update.len() / 2;
            let middle = &update[middle_index];
            // dbg!(middle_index, middle);
            sum_middle_index += middle.parse::<u32>().unwrap();
        } else {
            order(&instructions, update);
        }
    }

    dbg!(sum_middle_index);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_and_parse() {
        let s = read_input("src/day5/test.txt");
        let (instructions, updates) = parse_input(s);

        assert_eq!(updates[0], "75,47,61,53,29");
        assert_eq!(updates.last().unwrap(), "97,13,75,29,47");
        assert_eq!(instructions.first().unwrap(), "47|53");
        assert_eq!(instructions.last().unwrap(), "53|13");
    }

    #[test]
    fn test_parse_instructions() {
        let s = read_input("src/day5/test.txt");
        let (instructions, updates) = parse_input(s);
        let instructions = parse_instructions(instructions);

        assert!(instructions["75"].more.contains("97"));
        assert!(instructions["75"].less.contains("61"));
    }

    #[test]
    fn test_evaluate() {
        let s = read_input("src/day5/test.txt");
        let (instructions, updates) = parse_input(s);
        let instructions = parse_instructions(instructions);

        let updates: Vec<Vec<String>> = updates
            .into_iter()
            .map(|item| item.split(",").map(|i| i.to_owned()).collect())
            .collect();

        let result = evaluate(&instructions, &updates[0]);
        assert_eq!(result, true);

        let result = evaluate(&instructions, &updates[1]);
        assert_eq!(result, true);

        let result = evaluate(&instructions, &updates[2]);
        assert_eq!(result, true);

        let result = evaluate(&instructions, &updates[3]);
        assert_eq!(result, false);

        let result = evaluate(&instructions, &updates[4]);
        assert_eq!(result, false);

        let result = evaluate(&instructions, &updates[5]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_case_full() {
        let s = read_input("src/day5/test.txt");
        let (instructions, updates) = parse_input(s);
    
        let instructions = parse_instructions(instructions);
        let updates = parse_updates(updates);
    
        let mut sum_middle_index = 0u32;
    
        for update in updates.iter() {
            if evaluate(&instructions, update) {
                dbg!(update);
                let middle_index = update.len() / 2;
                let middle = &update[middle_index];
                dbg!(middle_index, middle);
                sum_middle_index += middle.parse::<u32>().unwrap();
            } 
        }
    
        assert_eq!(sum_middle_index, 143);
    }
}
