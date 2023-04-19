#[derive(Debug)]

struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i: ImportantExcerpt = ImportantExcerpt {
        part: first_sentence
    };

    println!("i is {:#?}", i);

    println!("level is {}", i.level());

    let announcement = String::from("announcement");

    i.announce_and_return_part(&announcement);
    
    println!("{}", lifetime_syntax_02::first_word(&i.part));

    {
        let words = String::from("hello world");

        println!("words is {words}");
    }

    let str1 = String::from("hello world");
    let str2 = String::from("hello world!");

    println!("the longest str is {}", lifetime_syntax_02::longest(&str1, &str2));

    let s: &'static str = "I have a static lifetime.";
    println!("words is {s}");

    println!("longest with an announcement is {}", lifetime_syntax_02::longest_with_an_announcement(&str1, &str2, 32));
}
