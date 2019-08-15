use std::thread;
use std::sync::mpsc;
fn main() {
    let (s, r) = mpsc::channel();
    let thread = thread::spawn(|| {
        run(s);
        run1(r);
    });

    thread.join().unwrap();

}

fn run(s: mpsc::Sender<i32>) {
    s.send(5).unwrap();
    s.send(3).unwrap();
}

fn run1(r: mpsc::Receiver<i32>) {
    let rec = r.recv().unwrap();
    println!("{}", rec);
    println!("{}", r.recv().unwrap());
}
