#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}
#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona,
}
fn value_in_cent(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state->{:?}",state);
            25
        },
    }
}
fn main() {
    println!("{:?}", value_in_cent(Coin::Penny));
    println!("{:?}", value_in_cent(Coin::Quarter(UsState::Alaska)));
}
