fn main() {
    println!("Hello, world!");

    // signed i[8 | 16 | 32 | 64 | 128 | size] 有符号整数
    // unsigned 无符号整数
    let a: i128 = -170141183460469231731687303715884105728;
    let b = 98_222;
    println!("{a}, {b}");

    // IEEE-754
    // 浮点 f32 单精度
    // 浮点 f64 双精度
    let x = 0.6;
    let y: f32 = 0.3;
    println!("{x}, {y}");

    // add
    let sum = 5 + 8;

    // sutract
    let difference = 9.1 - 7.2;

    // multiply
    let mul = 9.2 * 8.8;

    // divid
    let di = 8.8 / 1.1;

    // remainder
    let remainder = 10 % 3;

    println!("{sum}, {difference}, {mul}, {di}, {remainder}");

    // boole
    let t = true;
    let f: bool = false;

    println!("{t}, {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c}, {z}, {heart_eyed_cat}");

    // 元组 tuple
    let tup: (i32, u8, u16) = (-9, 32, 16);
    let (x, y, z) = tup;
    let first = tup.0;

    println!("tup le{x}, {y}, {z}, first: {first}");

    // 数组 array
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let b = [3; 5];

    println!("array a: {0}", a[4]);
    println!("array months: {0}", months[2]);
    println!("array b: {0}", b[2]);
    println!("Base 2 (binary): {:b}", 69420);

}
