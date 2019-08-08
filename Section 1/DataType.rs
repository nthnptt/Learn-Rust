fn main() {
    // (warning) out of range so a=0 (%256)
    let a:u8=256;
    let a:u8=1;
    // let a:u32=-1; => doesn't works
    println!("Hellow {}", a);

    let b:f32=258.3;
    println!("Hellow {}", b);

    let c:bool=true;
    println!("Hellow {}", c);

    let d:char='a';
    println!("Hellow {}", d);

}
