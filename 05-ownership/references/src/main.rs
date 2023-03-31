fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("s1 is {}, length is {}!", s1, len);
    
    let mut s2 = String::from("Hello");
    change(&mut s2);

    println!("s2 is {}", s2);

    let mut s3 = String::from("hello");
    let r1 = &mut s3;

    println!("r1 is {}", r1);

    let r2 = &mut s3;

    println!("r2 is {}", r2);

    let mut s5 = String::from("hello");
    let c1 = &s5;
    let c2 = &s5;

    println!("c1 is {}, c2 is {}", c1, c2);

    let c3 = &mut s5;
    println!("c3 is {}", c3);

    let reference_to_nothing = dangle();

    println!("reference_to_nothing is {}", reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
