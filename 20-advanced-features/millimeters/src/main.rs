use std::ops::Add;

struct Meters(u32);

#[derive(Debug)]
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn main() {
    let a = Millimeters(1);

    let b = a.add(Meters(5));

    println!("millimeters!, {:#?}", b);    
}
