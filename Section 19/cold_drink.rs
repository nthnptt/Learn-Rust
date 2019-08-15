use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn bottle(s: mpsc::Sender<&str>) {
    for i in 0..6 {
        println!("Building Bottle {}", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Bottles are ready");
    s.send("Bottle").unwrap();
}

fn cold_drink(s: mpsc::Sender<&str>) {
    let mut i = 5;
    while i > 0 {
        println!("Creating Cold Drink. Hours Left {}", i);
        thread::sleep(Duration::from_millis(1));
        i-=1;
    }
    println!("Cold drink are ready");
    s.send("Cold").unwrap();
}

fn seller(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "Bottle" && r.recv().unwrap() == "Cold" {
        println!("Thank for doing on time");
        println!("I will sell cold drink now");
    }
}

fn main() {
    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);

    let handle = thread::spawn(||{
        bottle(s1);
        cold_drink(s);
        seller(r);
    });

    handle.join().unwrap();
}
