#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.h * self.w
    }
}

fn main() {
    let rect = Rectangle {
        h:5,
        w:2
    };
    println!("{}", rect.area());
}
