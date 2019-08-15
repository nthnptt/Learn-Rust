fn main() {
    let x=1;

    match x {
        1 | 2=> println!("one or 2"),
        3=> println!("three"),
        _=> println!("other value")
    }

    match x {
        1...5 => println!("One - Fixe"),
        _ => println!("other")
    }

    let y = 'c';

    match y {
        'a'...'g' => println!("a..g"),
        _ => println!("Other")
    }
}
