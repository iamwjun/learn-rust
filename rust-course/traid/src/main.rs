use std::fmt::{Debug, Display};
use std::convert::TryInto;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notice<T: Summary>(item: &T) {
    println!("T news! {:#?}", item.summarize());
}

fn some_fn<T, U>(a: &T, b: &U) -> String
where
    T: Summary + Clone,
    U: Clone + Debug,
{
    format!("where")
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let post = Post {
        title: String::from("早安"),
        author: String::from("kk"),
        content: String::from("早餐"),
    };
    println!("{}", post.summarize());

    let weibo = Weibo {
        username: String::from("mm"),
        content: String::from("被发现了"),
    };
    println!("{}", weibo.summarize());

    notify(&Post {
        title: String::from("早安"),
        author: String::from("kk"),
        content: String::from("早餐"),
    });

    notice(&Post {
        title: String::from("早安"),
        author: String::from("kk"),
        content: String::from("早餐"),
    });

    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into()
                .unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
