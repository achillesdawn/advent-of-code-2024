// mod day4;
mod day8;

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).expect(&format!("could not find path: {}", path))
}

fn main() {
    day8::day_8();
}
