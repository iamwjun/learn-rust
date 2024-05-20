#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

trait NewIpAddr {
    fn display(&self);
}

struct V4 (String);

impl NewIpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}

struct V6 (String);

impl NewIpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
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

    let v: Vec<Box<dyn NewIpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v {
        ip.display()
    }

    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    println!("Vector's length is {}, capacity is {}", v.len(), v.capacity());

    v.reserve(100);
    println!("Vector's length is {}, capacity is {}", v.len(), v.capacity());

    v.shrink_to_fit();
    println!("Vector shrink_to_fit's length is {}, capacity is {}", v.len(), v.capacity());

    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    // add Ord, Eq, PartialEq, PartialOrd
    people.sort_unstable();
    println!("{:?}", people);
}

fn show_addr(ip: IpAddr) {
    println!("{:#?}", ip);
}
