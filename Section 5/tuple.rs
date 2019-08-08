fn main() {
    let a: (i32, bool, f64) = (254, true, 234.0);
    print_tuple(a);
}

fn print_tuple(t: (i32, bool, f64)) {
    let (var_a, var_b, var_c) = t;
    println!("{} {} {}", var_a, var_b, var_c);
}
