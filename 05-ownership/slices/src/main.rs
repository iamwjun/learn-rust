fn main() {
    let mut s1 = String::from("Hello, world!");
    let f = first_world(&s1);
    s1.clear();
    println!("s1's first world is {}, s1 curr is {}", f, s1);

    let s2 = String::from("Hello, world!");
    let c1 = &s2[0..5];
    let c2 = &s2[7..12];
    println!("c1 is {}, c2 is {}", c1, c2);

    let s3 = String::from("Hello, world!");
    println!("first word is {}", first_world_v2(&s3));

    let s4 = String::from("Hello, world!");
    let s5 = "Hello, world!";
    let s6 = &s4[..5];
    let s7 = &s5[..5];

    println!("s6 is {}, s7 is {}", s6, s7);

    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];

    assert_eq!(b, &[2, 3]);
}

fn first_world_v2(s: &String) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_world(s: &String) -> usize {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
