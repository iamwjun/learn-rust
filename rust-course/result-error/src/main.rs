use std::net::IpAddr;

fn main() {
    let v = vec![1, 2, 3];

    // std::panic::panic_any("burn");

    let home: IpAddr = "127.0.01".parse().unwrap();
    println!("home ip is {}", home);
}
