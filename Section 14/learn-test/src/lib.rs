struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length>other.length && self.width>other.width
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{length: 5, width: 3};
        let smaller = Rectangle{length: 2, width: 1};
        assert!(larger.can_hold(&smaller))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{length: 5, width: 3};
        let smaller = Rectangle{length: 2, width: 1};
        assert!(!smaller.can_hold(&larger))
    }
}
