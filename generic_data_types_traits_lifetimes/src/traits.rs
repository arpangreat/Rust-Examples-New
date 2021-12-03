pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Newspaper {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Newspaper {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
