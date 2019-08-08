#[derive(Debug)]
struct Student {
    name: String,
    rust: u32,
    c: u32,
    js: u32,
}

impl Student {
    fn build(name: String, rust: u32, c: u32, js: u32) -> Student {
        Student {
            name,
            rust,
            c,
            js,
        }
    }
    fn highest(&self){
        if self.c > self.js && self.c > self.rust {
            println!("C");
        }
        if self.js > self.c && self.js > self.rust {
            println!("Js");
        } else {
            println!("Rust");
        }
    }
}

fn main() {
    let student = Student::build(String::from("Joe"), 2, 3, 4);
    student.highest();
}
