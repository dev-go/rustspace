use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        format!("READ MORE: {}", self.summarize())
    }
}

#[derive(Debug)]
pub struct News {
    headline: String,
    content: String,
    author: String,
    location: String,
}

impl News {
    pub fn create(headline: String, content: String, author: String, location: String) -> News {
        News {
            headline,
            content,
            author,
            location,
        }
    }
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{}: {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Tweet {
    pub fn create(username: String, content: String, reply: bool, retweet: bool) -> Tweet {
        Tweet {
            username,
            content,
            reply,
            retweet,
        }
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("[notify] Breaking News: {}", item.summarize());
}

pub fn notify1<T: Summary>(item: T) {
    println!("[notify1] Breaking News: {}", item.summarize());
}

pub fn notify2<T: Summary + std::fmt::Display>(item: T) {
    println!("[notify2] Breaking News: {}", item.summarize());
}

pub fn notify3<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("[notify3] Breaking News: {}", a.summarize())
}
