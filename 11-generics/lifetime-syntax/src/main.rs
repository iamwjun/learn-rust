fn main() {
    let r;

    let x = 5;
    r = &x;

    println!("r is {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
