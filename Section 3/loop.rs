fn main() {
    let mut n= 0;
    loop { // infinite loop
        if n<5 {
            println!("Hello");
            n+=1;
        } else {
            break;
        }
    }
}