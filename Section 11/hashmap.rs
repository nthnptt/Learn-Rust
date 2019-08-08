use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();
    score.insert("Red", 10);
    score.insert("Blue", 20);
    println!("score->{:?}",score)
}
