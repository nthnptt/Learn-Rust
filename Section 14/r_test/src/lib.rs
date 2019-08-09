#![allow(dead_code)]
fn print_and_return(a: i32) -> i32 {
    println!("{}", a);
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let value = print_and_return(10);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value = print_and_return(10);
        assert_eq!(1, value);
    }
}
