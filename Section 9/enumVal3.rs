#[derive(Debug)]
enum IpType {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpType::V4(127, 0, 0, 1);
    let loopback = IpType::V6(String::from("::1"));
    println!("home->{:?}", home);
    println!("loopback->{:?}", loopback);
}
