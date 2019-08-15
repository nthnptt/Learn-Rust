fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let mut stack2 = vec!["a", "b", "c"];
    for (index, value) in stack2.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
