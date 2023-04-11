fn main() {
    let data = "initial contents";

    println!("data is {data}!");

    let s = data.to_string();

    println!("s is {s}!");

    let m = "initial contents".to_string();

    println!("m is {m}!");

    let x = String::from("initial contents");
    
    println!("x is {x}!");

    let hello = String::from("السلام عليكم");
    println!("hello is {hello}!");

    let hello = String::from("Dobrý den");
    println!("hello is {hello}!");

    let hello = String::from("Hello");
    println!("hello is {hello}!");

    let hello = String::from("שָׁלוֹם");
    println!("hello is {hello}!");

    let hello = String::from("नमस्ते");
    println!("hello is {hello}!");

    let hello = String::from("こんにちは");
    println!("hello is {hello}!");

    let hello = String::from("안녕하세요");
    println!("hello is {hello}!");

    let hello = String::from("你好");
    println!("hello is {hello}!");

    let hello = String::from("Olá");
    println!("hello is {hello}!");

    let hello = String::from("Здравствуйте");
    println!("hello is {hello}!");

    let hello = String::from("Hola");
    println!("hello is {hello}!");

    let mut hello = String::from("hello");
    hello.push_str(" world!");

    println!("str is {hello}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}, s1 is {s1}");

    let mut l = String::from("lo");
    l.push('l');
    println!("l is {l}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");

    let hello = String::from("Здравствуйте");
    println!("hello length is {}", hello.len());

    let s = &hello[0..4];
    println!("s is {s}");

    let s = &hello[0..2];
    println!("s is {s}");

    for c in "Зд".chars() {
        println!("c is {c}");
    }

    for b in "Зд".bytes() {
        println!("b is {b}");
    }
}
