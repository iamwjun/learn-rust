#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    dbg!(v);

    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("third is {}", third);

    match v.get(2) {
        Some(m) => println!("third is {}", m),
        None => println!("third is null"),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("The first element is: {first}");

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10
    }
    println!("The v is: {:#?}", v);

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];
    for ip in v {
        show_addr(ip);
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:#?}", ip);
}
