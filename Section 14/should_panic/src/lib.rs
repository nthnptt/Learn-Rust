struct Guess {
    value: i32
}
impl Guess {
    fn new(value: i32) -> Guess {
        if value<1 {
            panic!("Guess must be greater than 1");
        } else if value>100 {
            panic!("Guess must be smaller than 100");
        }
        Guess {value}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected="Guess must be smaller than 100")]
    fn new_guess_should_panic() {
        Guess::new(110);
    }
}
