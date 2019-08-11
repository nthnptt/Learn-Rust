pub fn add_one(x: u32) -> u32 {
    x+1
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_add_one() {
        let five = 5;
        assert_eq!(6, add_one(five));
    }
}
