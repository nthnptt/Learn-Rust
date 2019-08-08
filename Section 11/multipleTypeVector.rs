#[derive(Debug)]
enum SpreadSheet {
    Integer(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadSheet::Integer(32),
        SpreadSheet::Float(42.42),
        SpreadSheet::Text(String::from("Hello !"))
    ];
    println!("row->{:?}",row)
}
