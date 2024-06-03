use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let long_life: &'static str = "I have a long life time";

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("part is {:?}", i.part);
    println!("announce is {:?}", i.announce_and_return_part(&novel));

    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let result;

    {
        let string2 = String::from("long string is abcdef");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let s = String::from("abcd 1234");
    println!("result is {}", first_word(&s));

    println!("long_life's mean is {}", long_life);

    let x = String::from("x");
    let y = String::from("y");
    let ann = String::from("T");
    println!("longest is {}", longest_with_an_announcement(&x, &y, ann))
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn useless<'a>(a: &'a str, b: &'a str) {}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where 
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
