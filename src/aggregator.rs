pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Toot {
    pub username: String,
    pub mastodon_instance: String,
    pub content: String,
    pub is_reply: bool,
    pub is_reblog: bool,
}

impl Summary for Toot {
    fn summarize(&self) -> String {
        format!("@{}@{}: {}", self.username, self.mastodon_instance, self.content)
    }
}
