use std::collections::HashMap;
use std::fmt;

struct Person {
    name: String,
    age: i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

fn main() {
    let mut people = HashMap::new();

    people.insert("Alice", Person { name: String::from("Alice"), age: 30 });
    people.insert("Bob", Person { name: String::from("Bob"), age: 35 });

    for (name, person) in &people {
        println!("{}: {}", name, person);
    }
}
