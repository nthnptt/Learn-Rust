fn main() {
    let mut name = Some(String::from("John"));
    match name {
        Some(ref mut name) => *name= String::from("ABC"),
        None => ()
    }

    println!("{:?}", name);
}
