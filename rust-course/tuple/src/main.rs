fn main() {
    let tup: (u8, i32, f64) = (8, 32, 9000.00);
    dbg!(tup);
    dbg!(tup.1);

    let (x, y, z) = tup;
    dbg!(x, y, z);

    let str = String::from("hello");
    let (s, len) = calculate_length(str);
    println!("str is {}, str length is {}", s, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}
