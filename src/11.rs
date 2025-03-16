use std::fs;

fn main() {
    let data = fs::read_to_string("file.txt").unwrap();
    println!("{}", data);
}
