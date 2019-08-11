fn main() {
    let x = 4;
    let c = |z| z==x;
    let x = 7;
    let y = 4;
    assert!(c(y));
}
