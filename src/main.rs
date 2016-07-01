extern crate hyper;

use hyper::client::Client;
use std::io::Read;

fn main () {
    let reddit_content = get_url_content("https://www.reddit.com/r/programming/.rss");

    println!("Result: {}", reddit_content);    
}

fn get_url_content(url: &str) -> String {
    let client = Client::new();
    let mut s = String::new();
    client.get(url)
        .send()
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}