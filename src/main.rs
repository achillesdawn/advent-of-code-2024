// mod day4;
// mod day8;
mod day9;

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).expect(&format!("could not find path: {}", path))
}

fn main() {
    day9::day_9();
}
