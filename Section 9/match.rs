#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}
fn value_in_cent(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    println!("{}", value_in_cent(Coin::Penny));
    println!("{}", value_in_cent(Coin::Quarter));
}
