pub trait Summary {
    fn summerize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summerize(&self) -> String {
        format!("{}", self.headline)
    }
}
fn main() {
    println!("Hello, world!");
}
