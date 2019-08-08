#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpType,
    addr: String,
}

fn main() {
    let home = IpAddr {
        kind: IpType::V4,
        addr: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpType::V6,
        addr: String::from("::1"),
    };
    println!("home->{:?}", home);
    println!("loopback->{:?}", loopback);
}
