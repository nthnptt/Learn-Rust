use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (sender, receiver) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec!["more", "message", "for", "u"];

        for val in vals {
            s1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in receiver {
        println!("Got {}", rec);
    }
}
