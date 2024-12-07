
mod day4;
mod day6;
mod day7;

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).expect(&format!("could not find path: {}", path))
}

fn main() { 
    // day6::day_6_problem_one();
    day7::day7_problem_two();
    

}