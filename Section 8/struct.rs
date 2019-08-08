#[derive(Debug)]
struct User {
    username: String,
    age: i32,
}
fn build(name:String, age:i32) -> User {
    User {
        age,
        username: name,
    }
}
fn main() {
    let mut user = build(String::from("Test"), 32);
    println!("{:?}", user);
    user.age = 33;
    println!("{:?}", user);
}
