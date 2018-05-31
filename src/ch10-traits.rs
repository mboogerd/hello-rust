
use std::cmp::Ordering;

pub trait Summarizable {
    // method signature followed by semicolon, no impl.  
    fn summary(&self) -> String;
    // We are allowed to provide an impl, which will then
    // be the default.

    // Implementations can call other methods in the same
    // trait, regardless of whether they have a default impl. 
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/* Trait implementation
We can only implement traits for types, iif:
- the trait is in our crate, or
- the type is in our crate

i.e. we cannot implement an externally available trait for
an externally available type, including the standard lib.
See the orphan rule
*/
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Pair<T, U> {
    x: T,
    y: U
}

impl<T: PartialEq, U: PartialEq> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Pair<T, U>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialOrd, U: PartialOrd> PartialOrd for Pair<T, U> {
    fn partial_cmp(&self, other: &Pair<T, U>) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(x) => Some(x),
            None => self.y.partial_cmp(&other.y)
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}