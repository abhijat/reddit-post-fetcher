#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate reqwest;

use std::io::Read;

#[derive(Deserialize, Debug)]
struct QueryResult {
    data: QueryResultData
}

#[derive(Deserialize, Debug)]
struct QueryResultData {
    children: Vec<Post>
}

#[derive(Deserialize, Debug)]
struct Post {
    data: PostData
}

#[derive(Deserialize, Debug)]
struct PostData {
    url: String,
    selftext: String,
    author: String,
    title: String,
}

impl std::fmt::Display for PostData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} [{}] \n {}", self.title, self.author, self.url)
    }
}

fn get_results() -> std::io::Result<()> {
    let url = "https://www.reddit.com/r/askhistorians.json";

    let mut resp = reqwest::get(url).expect("unable to fetch the url!");
    assert!(resp.status().is_success());

    let mut buffer = String::new();
    resp.read_to_string(&mut buffer)?;

    let result: QueryResult = serde_json::from_str(&buffer)?;

    result.data.children
        .iter()
        .filter(|p| !p.data.selftext.is_empty())
        .for_each(|post| {
            println!("=> {}\n", post.data);
            println!("{}\n", post.data.selftext);
        });
    Ok(())
}

fn main() {
    let _ = get_results();
}