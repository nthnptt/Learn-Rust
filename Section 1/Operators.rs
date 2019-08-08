fn main() {
    let mut a = 4;
    let b = 3;
    let temp = a;
    println!("a:{} b:{}", a, b);
    println!("+ : {}", a+b);
    println!("- : {}", a-b);
    println!("/ : {}", a/b);
    println!("* : {}", a*b);
    println!("% : {}", a%b);
    a+=b;
    println!("+= : {}", a);
    a-=b;
    println!("-= : {}", a);
    a/=b;
    println!("/= : {}", a);
    a*=b;
    println!("*= : {}", a);
    a=temp;
    println!("> : {}", a>b);
    println!("< : {}", a<b);
    println!(">= : {}", a>=b);
    println!("<= : {}", a<=b);
    println!("== : {}", a==b);
    println!("&& : {}", true&&true);
    println!("|| : {}", true||true);
    println!("!= : {}", true!=true);
}