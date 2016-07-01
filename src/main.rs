extern crate hyper;

use hyper::client::Client;
use std::io::Read;

fn main () {

    let client = Client::new();
    let mut s = String::new();

    client.get("https://www.reddit.com/r/programming/.rss")
        .send()
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    println!("Result: {}", s);
    
}