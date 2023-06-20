use std::thread;

fn main() {
    type Kilometers = i32;

    let x: i32 = 6;
    let y: Kilometers = 8;

    println!("x + y is {}", x + y);

    let f: Box<dyn Fn() + Send +'static> = Box::new(|| println!("hi!"));

    type Thunk = Box<dyn Fn() + Send +'static>;

    fn take_long_type(_f: Thunk) {
        // snip
    }

    take_long_type(f);

    // closure
    fn returns_long_type() -> Thunk {
        // snip
        Box::new(|| println!("hi thunk!"))
    }

    returns_long_type()();

    let long = returns_long_type();

    long();

    let message = || {
        println!("message")
    };

    let boxed_closure: Thunk = Box::new(message);

    boxed_closure();

    thread::spawn(move || {
        boxed_closure();
    })
    .join()
    .expect("Thread panicked!");
}
