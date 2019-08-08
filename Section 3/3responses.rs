use std::io;

fn main() {
    println!("Hi guys ! Give me the name of an animal with a long neck ? (3 times)");
    let response = "girafe";
    let mut input = String::new();
    for _i in 0..3 {
        println!("Response :");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let input = input.trim().to_string();
        if input==*response {
            println!("You win !");
            return;
        } else {
            println!("Try again !");
        }
    }
    println!("You loose !");
}
