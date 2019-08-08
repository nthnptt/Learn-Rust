fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(15);
    v.push(60);
    println!("{:?}", v);
    let v2 = vec![10,12,30];
    println!("{:?}", v2);
//    let _v3 = Vec::new(); error
    let _v3 : Vec<i32> = Vec::new();

    let val = v2[0];
    println!("val->{}",val);

    let gval = v2.get(100);
    println!("gval->{:?}",gval) // get return Option
}
