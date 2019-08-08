#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.h * self.w
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let rect1 = Rectangle {
        h:30,
        w:50
    };
    let rect2 = Rectangle {
        h:10,
        w:30
    };
    let rect3 = Rectangle {
        h:60,
        w:50
    };
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}
