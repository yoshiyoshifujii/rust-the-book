use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    fn notify2<T: Summary>(_item1: &T, _item2: &T) {
        unimplemented!()
    }

    fn notify3(_item: &(impl Summary + Display)) {
        unimplemented!()
    }

    fn notify4<T: Summary + Display>(_item: &T) {
        unimplemented!()
    }

    fn some_function1<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
        unimplemented!()
    }

    fn some_function2<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        unimplemented!()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

#[derive(Debug)]
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

fn returns_summarizable() -> impl Summary + Debug {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::sample_trait::{NewsArticle, Summary, Tweet, returns_summarizable};

    #[test]
    fn test_trait() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    #[test]
    fn test_news_article() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }

    #[test]
    fn test_return_impl() {
        let result = returns_summarizable();
        println!("{:?}", result)
    }
}

