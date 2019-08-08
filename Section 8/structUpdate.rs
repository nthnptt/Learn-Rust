#[derive(Debug)]
struct User {
    username: String,
    age: i32,
}

fn main() {
    let user = User {
        username: String::from("Bob"),
        age: 23,
    };
    let user2 = User {
        username: String::from("John"),
        ..user
    };
    println!("{:?}", user);
    println!("{:?}", user2);
}
