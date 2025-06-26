pub struct NewsArticle {
    pub author: String,
    pub title: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
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
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let article = NewsArticle { 
        author: String::from("John Doe"),
        title: String::from("Rust Traits"),
        content: String::from("Understanding traits in Rust is essential for writing idiomatic code."),
    };
    let tweet = Tweet {
        username: String::from("johndoe"),
        content: String::from("Learning Rust is fun!"),
        reply: false,
        retweet: false,
    };      
     
    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());
}
 