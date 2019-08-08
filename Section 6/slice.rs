fn main() {
    let main = String::from("Hello world");
    let part_a = &main[0..5];
    let part_b = &main[2..4];
    let array = [5, 5, 5, 5];
    let sub_array = &array[..3];
    println!("{}", part_a);
    println!("{}", part_b);
    println!("{:?}", sub_array);
}
