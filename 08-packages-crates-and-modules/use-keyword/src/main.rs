use rand::{self, Rng};
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let mut hm: HashMap<i8, i8> = HashMap::new();
    hm.insert(127, 2);

    println!("hm is {:#?}", hm);

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("secret number is {}", secret_number);
}
