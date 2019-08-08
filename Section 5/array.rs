fn main() {
    let mut a: [i32; 5] = [0; 5];
    a[2] = 5;
    print(a);
}

fn print(a: [i32; 5]) {
    println!("length->{}",a.len());
    for i in a.iter() {
        println!("{}", i);
    }
}

