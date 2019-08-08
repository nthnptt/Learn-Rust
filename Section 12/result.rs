use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => { panic!("File not found"); }
    };
    println!("{:?}", f);
}
