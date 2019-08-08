#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn build(w: u32, h: u32) -> Rectangle {
        Rectangle { w, h }
    }
}

fn main() {
    let rect = Rectangle::build(2,2);
    println!("{:?}", rect);
}
