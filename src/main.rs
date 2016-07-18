extern crate hyper;
extern crate xml;
extern crate time;

use hyper::client::Client;
use std::io::Read;
use time::{Tm, Duration};
use xml::reader::{EventReader, XmlEvent};


fn main () {
    let reddit_content = get_url_content("https://www.reddit.com/r/programming/.rss");
    println!("Result: {:?}", tm_lt_tz(time::now() + Duration::seconds(-500), "2016-07-05T22:15:24+00:00"));
    println!("Result: {:?}", tz_lt_tz("2015-05-12T22:15:24+05:53", "2016-07-10T22:15:24+00:00"));
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

fn tz_lt_tz(left: &str, right: &str) -> bool {
    time::strptime(left, "%Y-%m-%dT%T%z").unwrap() < time::strptime(right, "%Y-%m-%dT%T%z").unwrap()
}

fn tm_lt_tz(left: Tm, right: &str) -> bool {
    left < time::strptime(right, "%Y-%m-%dT%T%z").unwrap()
}