fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("{} {} {} {}", c, z, g, heart_eyed_cat);
    println!("字符'c'占用了{}字节的内存大小", std::mem::size_of_val(&c));

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}
