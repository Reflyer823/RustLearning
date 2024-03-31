use trait_demo::largest;
use trait_demo::notify;
use trait_demo::returns_summarizable;
use trait_demo::NewsArticle;
use trait_demo::Pair;
use trait_demo::Tweet;
use trait_demo::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(tweet);
    notify(article);

    let summarizable = returns_summarizable();
    notify(summarizable);

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    println!("The largest word is {}", result);

    let pair = Pair::new(3, 5);
    println!("{}", pair);
    pair.cmp_display();
}
