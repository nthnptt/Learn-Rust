use std::collections::HashMap;
use std::io;
fn main() {
    let data_name = vec![String::from("Bob"), String::from("John"), String::from("Lorem")];
    let data_phone_number = vec![String::from("0643342334"), String::from("0643342308"), String::from("0643342322")];
    let collection: HashMap<&String,&String> = data_name.iter().zip(data_phone_number.iter()).collect();
    loop {
        let mut choice=String::new();
        println!("Which Name to Search");
        io::stdin().read_line(&mut choice).expect("Fail");
        let choice:String=choice.trim().parse().expect("Fail");
        let number = search(&collection, &choice);
        println!("{}", number);
    }

}

fn search(database: &HashMap<&String, &String>, choice: &String) -> String {
    let research = database.get(choice);
    let result : String;
    match research {
        None => result=String::from("Not found"),
        Some(s) => result=s.to_string()
    }
    result
}
