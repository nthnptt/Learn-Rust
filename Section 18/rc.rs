enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("Counter after creating a : {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Counter after creating b : {}", Rc::strong_count(&a));
    {
        let c = Cons(6, Rc::clone(&a));
        println!("Counter after creating c : {}", Rc::strong_count(&a));
    }
    println!("Counter after dropping c : {}", Rc::strong_count(&a));
}
