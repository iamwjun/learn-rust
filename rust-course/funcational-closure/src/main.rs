use std::{thread, time::Duration};

struct Cacher<T, K>
where
    T: Fn(K) -> K,
{
    query: T,
    value: Option<K>,
}

impl<T: AsRef<K>, K> Cacher<T, K>
where
    K: Clone,
    T: Fn(K) -> K,
{
    fn new(query: T) -> Cacher<T, K> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: K) -> K {
        match &self.value {
            Some(v) => v.clone(),
            None => {
                let v = (self.query)(arg);
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn muuuuu(intensity: u32) -> u32 {
    println!("muuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("Do {} push-ups first", action());
        println!("Do {} more bench presses", action());
    } else if random_number == 3 {
        println!("rest!");
    } else {
        println!("Run for {} minutes", action());
    }
}

fn main() {
    let x = 3;
    let sum = |y| y + x;

    assert_eq!(5, sum(2));

    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
}

fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

fn exec<F: FnMut()>(mut f: F) {
    f()
}
