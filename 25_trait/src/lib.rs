use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        // String::from("(Read more ...)")
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    println!("{:?}", b);
    format!("Breaking news! {}", a.summarize())
}

pub fn notify5<T, U>(a: T, b: U) -> String
where T: Summary + Display, U: Clone + Debug,
{
    println!("{:?}", b);
    format!("Breaking news! {}", a.summarize())
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 为所有实现了Display Trait的T类型，给Pair<T>也实现Display Trait
impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pair: {}, {}", self.x, self.y)
    }
}
