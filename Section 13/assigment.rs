use std::ops::Add;
struct S <T:Add> {
    a: T,
    b: T
}

impl <T:Add> S<T> {
    fn add(self) -> T::Output{
        self.a+self.b
    }
}

fn main() {
    let s = S{a: 5, b: 3};
    println!("{}", s.add());
}


