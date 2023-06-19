use std::ops::Add;

pub trait Iterator<T> {
    type Item;

    fn next(&mut self) -> Option<T>;
}

pub struct Counter {
}

impl Iterator<u32> for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // snip
        Some(32)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 0, y: 0 } + Point { x: 1, y: 1 },
        Point { x: 1, y: 1 }
    );

    println!("p1 + p2 is {:#?}", Point { x: 0, y: 0 } + Point { x: 1, y: 1 });
}
