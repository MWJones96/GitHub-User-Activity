use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

fn build_url(username: &str) -> String {
    format!("https://api.github.com/users/{username}/events")
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
            for event in events {
                match event._type {
                    Some(_type) => match _type {
                        EventType::CommitCommentEvent => todo!(),
                        EventType::CreateEvent => todo!(),
                        EventType::DeleteEvent => todo!(),
                        EventType::ForkEvent => todo!(),
                        EventType::GollumEvent => todo!(),
                        EventType::IssueCommentEvent => todo!(),
                        EventType::IssuesEvent => todo!(),
                        EventType::MemberEvent => todo!(),
                        EventType::PublicEvent => todo!(),
                        EventType::PullRequestEvent => todo!(),
                        EventType::PullRequestReviewEvent => todo!(),
                        EventType::PullRequestReviewCommentEvent => todo!(),
                        EventType::PullRequestReviewThreadEvent => todo!(),
                        EventType::PushEvent => todo!(),
                        EventType::ReleaseEvent => todo!(),
                        EventType::SponsorshipEvent => todo!(),
                        EventType::WatchEvent => todo!(),
                    },
                    None => {}
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
