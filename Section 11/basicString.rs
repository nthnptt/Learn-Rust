fn main() {
    let a = 8;
    let mut s = a.to_string();
    println!("s->{}",s);
    s.push('a');
    println!("s->{}",s);
    s.push_str(" hello");
    println!("s->{}",s);
    let s2 = String::from(" World");
    let s3 = s + &s2; // move s, so can't use it after
    println!("s3->{}",s3);
    let s4 = format!("{}-{}", s2, s3);
    println!("s4->{}",s4);

}
