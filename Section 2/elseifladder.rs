fn main() {
    let a = 100;
    let b = 20;
    let c = 50;
    if a > b && a > c {
        println!("A is Greatest");
    } else if b>a && a > c {
        println!("B is Greatest");
    } else {
        println!("C is Greatest");
    }
}