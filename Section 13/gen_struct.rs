#[derive(Debug)]
struct Point<T, E> {
    x: T,
    y: E,
}

impl <T, E> Point<T, E> {
    fn x (&self) -> &T {
        &self.x
    }
}
fn main() {
    let integer = Point {x: 3, y:2};
    let float = Point {x: 3.2, y:2.3};
    let mix = Point {x: 3, y: 3.2};
    println!("integer->{:?}",integer.x());
    println!("float->{:?}",float);
    println!("mix->{:?}",mix);
}
