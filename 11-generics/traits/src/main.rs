use traits::{Summary, NewsArticle, Tweet, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("shanghai"),
        author: String::from("wjun"),
        content: String::from("content"),
    };

    println!("1 new article: {}", article.summarize_author());
    println!("1 new article: {}", article.summarize());

    notify(&article);

    notify(&tweet);
}
