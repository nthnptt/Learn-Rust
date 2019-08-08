fn main() {
    facto_print(6);
    println!("number_return->{}",facto_return(6));
    println!("number_fn->{}",facto_fn_inside(6));
    println!("number_double_return->{:?}", facto_doublturne_re(6));
}

fn facto_print(number: i32) {
    let mut result = 1;
    for i in 2..number + 1 {
        result *= i;
    }
    println!("result->{}",result)
}

fn facto_return(number: i32) -> i32 {
    let mut result = 1;
    for i in 2..number + 1 {
        result *= i;
    }
    result
}

fn facto_fn_inside(number: i32) -> i32 {
    fn facto(number: i32) -> i32 {
        let mut result = 1;
        for i in 2..number + 1 {
            result *= i;
        }
        result
    }
    facto(number)
}

fn facto_doublturne_re(number: i32) -> (String, i32) {
    let mut result = 1;
    for i in 2..number + 1 {
        result *= i;
    }
    (String::from("Factorial is :"), result)
}
