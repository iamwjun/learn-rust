trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < b.into() {
        println!("Ten is less than one hundred.");
    }

    let t: &mut i32 = &mut 0;
    foo(t);
}
