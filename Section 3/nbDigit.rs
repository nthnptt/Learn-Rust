use std::io;

fn main() {
    let mut number: f32;
    let mut count: u32 = 0;
    let mut input = String::new();
    println!("Give me a number");
    io::stdin().read_line(&mut input).expect("Failed");
    number = input.trim().parse().expect("Failed");
    while number >= 1.0 {
        number = number / 10.0;
        count += 1;
    }
    println!("Number of digit : {}", count);
}
