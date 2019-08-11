use std::thread;
use std::time::Duration;

fn main() {
    let expensive_clojure = |num: u32| -> u32{
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let add_two_v2 = |i: u32| -> u32 { i+1 };
//    let add_two_v3 = |i| -> u32 {i+1}; Error
//    let add_two_v4 = |i| {i+1}; Error
//    let add_two_v5 = |i| i+1; Error
    println!("{}",expensive_clojure(5));
}

fn add_one_v1(i: u32) -> u32{
    i+1
}
