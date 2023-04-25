use std::{time::Duration, thread};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

struct  Solution {}

impl Solution {
    pub fn fib(&self, n: i32) -> i32 {
        match n {
            0 | 1 => n,
            _ => self.fib(n - 1) + self.fib(n - 2)
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
      1 | 2 => 1,
      _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
  }

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };
    
    let user1 = Some(ShirtColor::Red);
    let give1 = store.giveway(user1);

    println!("user1 like {:?}, give him a {:?}!", user1, give1);

    let user2 = None;
    let give2 = store.giveway(user2);

    println!("user2 like {:?}, give him a {:?}!", user2, give2);

    let user3 = Some(ShirtColor::Blue);
    let give3 = store.giveway(user3);

    println!("user2 like {:?}, give him a {:?}!", user3, give3);

    let solution = Solution {};
    
    println!("fibonacci is {}", solution.fib(5));

    println!("fibonacci is {}", fibonacci(6));

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("expensive_closure {}", expensive_closure(6));
}
