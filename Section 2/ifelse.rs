fn main() {
//    ex1();
//    ex2();
    ex3();
}

fn ex1() {
    let a = 10;
    if a % 2 == 0 {
        print!("Even");
    } else {
        print!("Odd");
    }
}

fn ex2() {
    let mut ch = String::new();
    println!("Are your friends coming ?");
    io::stdin().read_line(&mut ch).expect("failed");
    let ch = ch.trim().to_string(); // rm \n
    if ch == "y" {
        println!("Yeah ! Going for movie");
    } else {
        println!("Stay at home");
    }
}

fn ex3() {
    let mut name = String::new();
    let mut ages = String::new();
    let mut ch = String::new();

    println!("Enter Name and Age :");
    io::stdin().read_line(&mut name).expect("Failed");
    io::stdin().read_line(&mut ages).expect("Failed");
    let age: i32 = ages.trim().parse().expect("Failed");

    println!("Do you want to create account ?");
    io::stdin().read_line(&mut ch).expect("Failed");
    ch = ch.trim().to_string();
    if ch != "y" {
        println!("Good bye");
        return;
    }
    if age < 10 {
        println!("Your age is less");
    } else {
        println!("Your Account is created");
        println!("Do you want to upload photo ?");
        ch.clear(); // because read append to ch, not erased
        io::stdin().read_line(&mut ch).expect("Failed");
        ch = ch.trim().to_string();
        if ch == "y" {
            if age > 13 {
                println!("Photo uploaded successfully");
            } else {
                println!("Photo upload forbidden");
            }
        }
    }
}