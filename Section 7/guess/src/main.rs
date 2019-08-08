extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number");
    let secret = rand::thread_rng().gen_range(1, 10);
    loop {
        println!("Number?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let guess: i32 = input.trim().parse().expect("Failed");

        if guess == secret {
            println!("Win !");
            break;
        } else {
            print!("Try again !");
            if guess > secret {
                println!("(Down)");
            } else {
                println!("(Up)");
            }
        }
    }
}
