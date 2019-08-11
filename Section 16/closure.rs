use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 5;
    let simulated_random_number = 3;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating Slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_struct = Cacher::new(|num| {
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 32 {
        println!("Today do {} Pushups", expensive_struct.value(intensity));
        println!("Today do {} Situps", expensive_struct.value(intensity));
    } else {
        if random_number == 3 {
            println!("Today take a break");
        } else {
            println!("Today run for {} minutes", expensive_struct.value(intensity));
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let result = (self.calculation)(arg);
                self.value = Some(result);
                result
            },
            Some(v) => v
        }
    }
}
