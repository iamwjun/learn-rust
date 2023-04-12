use std::vec;

fn main() {
    let number_list = vec![23, 56, 1, 100, 235];

    let mut largest = &number_list[0];

    for n in &number_list {
        if n > largest {
            largest = n;
        }
    }

    println!("largest number is {largest}!");

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", get_largest(&number_list));
}

fn get_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}