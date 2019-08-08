use std::io;
fn main() {
    let mut a=String::new();
    /*println!("Enter a string :");
    io::stdin().read_line(&mut a).expect("Failed");
    println!("{}", a);*/

    println!("Enter a nuber :");
    io::stdin().read_line(&mut a).expect("Failed");
    let a:bool=a.trim().parse().expect("Failed");
    println!("{}", a);
}