fn main() {
    let num = Some(5);
    let num_null: Option<u32> = None;
    let text: Option<&str> = None;
    println!("{:?} - {:?} - {:?}", num, num_null, text);
}
