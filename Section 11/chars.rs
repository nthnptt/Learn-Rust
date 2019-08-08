fn main() {
    let s1 = String::from("Hello");
    for c in s1.chars() {
        println!("c->{}",c)
    }
    println!("{}", &s1[0..1]);
}
