fn main() {
    let a = "Hello";
    let b = "Bye";
    let result = longest(a, b);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
