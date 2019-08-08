fn main() {
    let num = if 3<2 {
        println!("if block");
        1
    } else {
        println!("Else");
        2
    };
    println!("{}", num);
}