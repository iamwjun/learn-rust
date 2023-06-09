fn main() {
    let mut str = String::from("Hello");

    str.push_str(" world!");
    str.insert_str(0, "Insert ");
    
    println!("{str}");

    let s1 = String::from("hello");
    let _s2 = s1.clone();

    println!("{}, world!", s1);

    let x = 29;
    let y = x;

    println!("x = {:x}, y = {}", x, y);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
