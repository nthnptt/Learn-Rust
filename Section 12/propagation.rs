use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let out =read();
    let out = match out {
        Ok(fi) => println!("{:?}", fi),
        Err(err) => println!("{:?}", err)
    };
}
fn read() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s= String::new();
    match f.read_to_string(&mut s) {
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}
