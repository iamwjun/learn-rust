use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct PersonSe {
    name: String,
    age: u8,
}

impl fmt::Display for PersonSe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "brother {}, age {}",
            self.name, self.age
        )
    }
}

struct Array(Vec<i32>);

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}

fn main() {
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{value}", value = 4); // => "4"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{:04}", 42); // => "0042" with leading zeros

    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");

    eprintln!("Error: Could not complete task");

    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{:?}, {:?}, {:?}, {:#?}, {}, {}", i, s, v, p, p.name, p.age);

    let p = PersonSe {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);

    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);

    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");

    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
}
