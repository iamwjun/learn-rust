use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("home ip is {}, loop ip is {}", home, loopback);
}
