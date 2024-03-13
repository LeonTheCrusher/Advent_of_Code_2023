use std::fs;

fn main() {
    let content: String = fs::read_to_string("test.txt").unwrap();
}
