use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greating = File::open("hello.txt");

    let greating_file = match greating {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem create the file: {:?}", e),
            },
            other_error => {
                panic!("Problem open the file: {:?}", other_error);
            }
        },
    };

    println!("greating file is {:?}!", greating_file);

    let world_file = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|e| {
                panic!("Problem create the file: {:?}", e);
            })
        } else {
            panic!("Problem open the file: {:?}", error);
        }
    });

    println!("world file is {:?}!", world_file);

    // let test_file = File::open("test.txt").unwrap();

    // let demo_file = File::open("demo.txt").expect("demo.txt should be included in this project");

    // println!("demo file is {:?}!", demo_file);

    let user_name = read_username_from_file();
    println!("user_name is {:?}!", user_name);

    let user = read_user_from_file();
    println!("user is {:?}!", user);

    let user = read_user_from_file_v2();
    println!("user is {:?}!", user);

    let last_char = last_char_of_first_line("this is a test");
    println!("last char is {:?}!", last_char);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_user_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_user_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
