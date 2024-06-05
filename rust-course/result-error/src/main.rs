use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    println!("file is {:#?}", file);

    let content = read_username_from_file();
    println!("content is {:#?}", content);

    println!("open file is {:#?}", open_file());

    println!("read file is {:#?}", read_file());

    let text = String::from("iamw");
    println!("fist is {:#?}", last_char_of_first_line(&text));
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(f)
}

fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
