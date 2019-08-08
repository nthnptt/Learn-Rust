#[warn(dead_code)]
#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

fn main() {
    let four = IpType::V4;
    let six = IpType::V6;
    route(six);
    println!("{:?}", four);
}

fn route(ip: IpType) {
    println ! ("ip->{:?}", ip)
}
