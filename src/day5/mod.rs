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

pub fn day5_first() {
    let s = read_input("src/day5/test.txt");
    let (instructions, updates) = parse_input(s);

    

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

    }
}
