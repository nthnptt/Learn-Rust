use std::collections::HashMap;
fn main() {
    let mut score = HashMap::new();
    score.insert("Blue", 20);
    println!("score->{:?}",score);
    score.entry("Blue").or_insert(50);
    score.entry("Red").or_insert(50);
    println!("score->{:?}",score);
    score.insert("Blue", 60);
    println!("score->{:?}",score);
}
