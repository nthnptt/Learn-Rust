enum Fruit {
    Apple = 0,
    Mango = 10,
    Watermelon = 20
}

fn main() {
    let f = Fruit::Apple;
    println!("{}", f as i32);
}
