use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

fn build_url(username: &str) -> String {
    format!("https://api.github.com/users/{username}/events")
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    #[serde(rename = "type")]
    _type: Option<String>,
}

fn main() {
    let client = Client::builder()
        .user_agent("RustClient/1.0")
        .build()
        .unwrap();

    match client.get(build_url("torvalds")).send() {
        Ok(res) => {
            let json = res.text().unwrap();
            let events: Vec<Event> = serde_json::from_str(&json).unwrap();
            println!("{:?}", events);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
