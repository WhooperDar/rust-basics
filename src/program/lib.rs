pub trait Summarizable { 
    fn summary (&self) -> String;
}

// struct news article
pub struct NewsArticle { 
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String
}

// struct tweet
pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool
}

// implementations
impl Summarizable for Tweet { 
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summarizable for NewsArticle { 
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


