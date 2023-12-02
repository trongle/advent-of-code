use std::fs;

pub fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect(&format!("Could not open file {}", filename));
}
