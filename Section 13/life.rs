fn main() {
    let s = String::from("Hello");
    let r = word(&s);
    println!("r->{}", r);
}

fn word(s: &str) -> &str {
    s
}
