
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(3, 4);

    for (key, value) in &map {
        println!("{} {}", key, value);
    }
}