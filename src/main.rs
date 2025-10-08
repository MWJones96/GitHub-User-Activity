use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

fn build_url(username: &str) -> String {
    format!("https://api.github.com/users/{username}/events")
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {}

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Actor {
    login: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum EventType {
    CommitCommentEvent,
    CreateEvent,
    DeleteEvent,
    ForkEvent,
    GollumEvent,
    IssueCommentEvent,
    IssuesEvent,
    MemberEvent,
    PublicEvent,
    PullRequestEvent,
    PullRequestReviewEvent,
    PullRequestReviewCommentEvent,
    PullRequestReviewThreadEvent,
    PushEvent,
    ReleaseEvent,
    SponsorshipEvent,
    WatchEvent,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    #[serde(rename = "type")]
    _type: Option<EventType>,
    actor: Actor,
    repo: Repo,
    payload: Payload,
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
