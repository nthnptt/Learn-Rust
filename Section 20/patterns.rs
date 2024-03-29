fn main() {
    let favourite_color: Option<&str> = None;
    let is_thusday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite color {} as the background", color);
    } else if is_thusday {
        println!("Tuesday is green day !");
    } else if let Ok(age) = age {
        if age>30 {
            println!("Using purple as background-color");
        } else {
            println!("Using orange as background-color");
        }
    } else {
        println!("Using blue as background-color");
    }
}
