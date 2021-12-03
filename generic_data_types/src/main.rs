use generic_data_types::{self, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Swastik"),
        content: String::from("Welcome , Welcome mf , The king is back"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
