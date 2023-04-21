enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveway() -> ShirtColor {
        
    }

    fn most_stocked() -> ShirtColor {
        ShirtColor::Red
    }
}

fn main() {
    println!("Hello, world!");
}
