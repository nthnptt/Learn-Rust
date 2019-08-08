const C:i32 = 32;
fn main() {
    let a=10; // unmutabale value
    // a=20; conpile error
    let mut b=10;
    println!("Hellow {}-{}", a, b);
    b=20;
    println!("Hellow {}-{}", a, b);
    // const C = 32; !error const need a type
    println!("End {}", C);

    let s="Hey";
    let s2 = String::from("World !");
    // let mut s3 = String::new();
    let mut s3 = String::from("!");
    s3.push_str("Hello !");
    // s3 = "!"; Error
    println!("{} {} {}", s, s2, s3)
}