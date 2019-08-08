use std::cmp::Ordering;

// from documentation
#[derive(Debug, Copy, Clone)]
struct Integer {
    number: u32,
}
// from documentation
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}
// from documentation
impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

fn main() {
    let list = vec![32,43,23,12];
    let listc = vec!['a','b','d','a'];
    let listu = vec![Integer{number: 3}, Integer{number: 15}, Integer{number: 9}];

    println!("largest->{}",largest(&list));
    println!("largest->{}",largest(&listc));
    println!("largest->{:?}",largest(&listu));
}

fn largest<T:PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &n in list {
        if n>largest {
            largest=n;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &n in list {
        if n>largest {
            largest=n;
        }
    }
    largest
}
