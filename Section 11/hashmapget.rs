use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 12);
    scores.insert("Yellow", 1);
    scores.insert("Red", 21);
    println!("{:?}", scores.get("Blue"));
    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }
}
