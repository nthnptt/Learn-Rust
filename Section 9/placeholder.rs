fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        _ => { //default
            println!("all");
            None
        }
    }
}