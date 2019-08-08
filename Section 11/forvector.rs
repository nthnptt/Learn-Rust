fn main() {
    let mut v=vec![1,2,3,4];
    for i in &mut v {
        *i*=2;
        println!("i->{}",i)

    }
}
