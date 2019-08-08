struct Point<T> {
    x: T,
}

impl Point<f32> {
    fn number(&self) -> f32 {
        self.x
    }
}

impl Point<u32> {
    fn number(&self) -> u32 {
        self.x
    }
}
fn main() {
    let n= Point{x:2.2};
    println!("n.number()->{}",n.number());

    let n= Point{x:2};
    println!("n.number()->{}",n.number());
}
