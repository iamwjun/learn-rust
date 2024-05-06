fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("number is {}", number);

    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    for i in 1..=5 {
        if i == 2 {
            continue;
        }
        println!("the value of i is {}", i);
        if i == 3 {
            break;
        }
    }

    let mut n = 0;

    while n <= 5  {
        println!("n is {} in while!", n);

        n = n + 1;
    }

    let mut m = 0;
    loop {
        if m == 3 {
            break;
        }

        println!("the value of m is {} in loop", m);
        m += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the value of counter is {}", result);
}
