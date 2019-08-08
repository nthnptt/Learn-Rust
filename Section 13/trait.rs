trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_auth())
    }
    fn summarize_auth(&self) -> String;

}

struct NewsArticle {
    headlinr: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_auth(&self) -> String {
        format!("{}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize_auth(&self) -> String {
        format!("@{}", self.author)
    }
}

fn main() {
    let news = NewsArticle {
        headlinr: String::from("We won"),
        location: String::from("Australia"),
        author: String::from("John"),
        content: String::from("We won World Cup"),
    };
    let tweet = Tweet {
        username: String::from("John"),
        content: String::from("We won the match"),
        reply: false,
        retweet: false,
    };
    println!("{}", news.summarize());
    println!("{}", tweet.summarize());
}
