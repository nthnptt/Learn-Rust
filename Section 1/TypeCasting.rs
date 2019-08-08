fn main() {
    let a:i32=10;
    let b:i64=a as i64;
    let b:i64=a as i64+ 10;
    let b:i64=(a + 10).into();
    let a:i64=a as i64+10;
    println!("a:{}, b:{}", a, b);
}