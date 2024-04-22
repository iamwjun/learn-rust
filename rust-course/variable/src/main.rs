use num::complex::Complex;

struct Struct {
    e: i32
}

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);

    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let guess = "42".parse::<i32>().expect("Not a number!");

    println!("guess is {}", guess);

    let m1 = 98_222;
    let m2 = 0x12;
    let m3 = 0o10;
    let m4 = 0b10000;

    println!("m is {} {} {} {}", m1, m2, m3, m4);

    assert_eq!(100u8.saturating_add(1), 101);

    println!("{}", 100u8.saturating_add(1));

    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    println!("u8 MAX is {}", u8::MAX.saturating_add(127));

    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}, {}", b, u8::MAX);  // 19

    let x = 3.1;
    let y: f32 = 9.22;
    println!("x is {}, y is {}", x, y);
	
	println!("0.1 + 0.2 = {}", 0.1 + 0.2);

	let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

	let x = (-42.0_f32).sqrt();
	println!("-42.0 sqrt is {}", x);
	if x.is_nan() {
		println!("未定义的数学行为");
	}

	let one_million = 1_000_000;
	println!("one million is {}", one_million);

	let forty_twos = [
		42.0,
		42f32,
		42.0_f32,
	];
	println!("{:.8}", forty_twos[0]);

	let a:i32 = 2;
    let b:i32 = 3;
	println!("a is {:b}", a);
	println!("b is {:b}", b);
	println!("a & b is {:b}", a & b);
	println!("a | b is {:b}", a | b);
	println!("a ^ b is {:b}", a ^ b);
	println!("!b is {:b}", !b);
	println!("a >> b is {:b}", a >> b);
	println!("a << b is {:b}", a << b);

	for i in 1..=5 {
		println!("i is {}", i);
	}

	for i in 'a'..='z' {
		println!("{}", i);
	}

	let a = Complex { re: 2.1, im: -1.2 };
	let b = Complex::new(11.1, 22.2);
	let result = a + b;

	println!("{} + {}i", result.re, result.im)
}
