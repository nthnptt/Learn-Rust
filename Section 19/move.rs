use std::thread;
fn main() {
    let v = vec![1,2,3];
    let thread = thread::spawn(move || {
        println!("{:?}", v);
    });
    thread.join().unwrap();
}
