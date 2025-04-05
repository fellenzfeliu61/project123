use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input file");
    println!("{}", data);
}
