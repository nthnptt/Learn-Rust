use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    *value.borrow_mut() += 5;
    println!("a after : {:?}", a);
    *value.borrow_mut() += 5;
    println!("b after : {:?}", b);
    *value.borrow_mut() += 5;
    println!("a value : {:?}", value);

}
