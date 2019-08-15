use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Hello from thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }

    });
    handle.join().unwrap();

    for i in 0..5 {
        println!("Hello form main {}", i);
        thread::sleep(Duration::from_millis(1));
    }

}
