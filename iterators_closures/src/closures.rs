use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use std::thread;
use std::time::Duration;

struct Cacher<T, I, O>
where
    I: Eq + Hash + Copy,
    T: Fn(I) -> O,
{
    calculation: T,
    values: HashMap<I, O>,
}

impl<T, I, O> Cacher<T, I, O>
where
    I: Eq + Hash + Copy,
    T: Fn(I) -> O,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: I) -> &O {
        match self.values.entry(arg) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let v = (self.calculation)(arg);
                e.insert(v)
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    #[test]
    fn cacher_u32_u32() {
        let mut cacher = Cacher::new(|x| x);

        let _v1 = cacher.value(1);
        let v2 = cacher.value(2);

        assert_eq!(*v2, 2)
    }

    #[test]
    fn cacher_str_usize() {
        let mut cacher = Cacher::new(|x: &str| x.len());

        let _v1 = cacher.value("hello");
        let v2 = cacher.value("foobar");

        assert_eq!(*v2, 6)
    }

    #[test]
    fn closure_capture() {
        let x = 4;

        let equal_to_x = |z| z == x;

        assert!(equal_to_x(4))
    }

    #[test]
    fn closure_capture_move() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;

        // println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y))
    }
}
