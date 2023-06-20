fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|i| i + 1)
}

fn main() {
    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let answer = do_twice(add_one, 6);

    println!("answer is {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    println!("the string list is {:?}", list_of_string);

    let string_vec: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("the string vec is {:?}", string_vec);
}