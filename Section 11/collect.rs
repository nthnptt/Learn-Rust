use std::collections::HashMap;
fn main() {
    let teams = vec!["Red", "Yellow", "Blue"];
    let score = vec![20, 5, 10];
    let hash : HashMap<_,_> = teams.iter().zip(score.iter()).collect();
    println!("hash->{:?}",hash);
}
