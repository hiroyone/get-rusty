use std::iter::Sum;

pub trait Summary {
    fn summarize (&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
} 

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize (&self) -> String {
        format!("{}: {}",  self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify (item: &impl Summary) {
    println!("Breaking News! {}", item.summarize())
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@sample"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@bianca"),
        content: String::from("Hello Rust!"),
        reply: false,
        retweet: false
    };
    let article = NewsArticle {
        author: String::from("Daniel"),
        headline: String::from("See what promise was broken!"),
        content: String::from("Nothing was broken!!"),
    };

    println!("The tweet summary: {}",tweet.summarize());
    println!("The tweet user: {}",tweet.summarize_author());
    println!("The article summary: {}",article.summarize());
    println!("The article author: {}",article.summarize_author());

    notify(&tweet);
}