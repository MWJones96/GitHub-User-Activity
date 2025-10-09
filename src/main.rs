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
struct Repo {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    #[serde(rename = "type")]
    _type: Option<EventType>,
    repo: Repo,
    payload: serde_json::Value,
}

fn main() {
    let client = Client::builder()
        .user_agent("RustClient/1.0")
        .build()
        .unwrap();

    match client.get(build_url("kamranahmedse")).send() {
        Ok(res) => {
            let json = res.text().unwrap();
            let events: Vec<Event> = serde_json::from_str(&json).unwrap();
            println!("Output:");
            for event in events {
                match event._type {
                    Some(_type) => match _type {
                        EventType::CommitCommentEvent => todo!(),
                        EventType::CreateEvent => {
                            let ref_type = event.payload.get("ref_type").unwrap().as_str().unwrap();
                            let repo_name = event.repo.name;
                            println!("{}", format!("- New {ref_type} was created in {repo_name}"));
                        }
                        EventType::DeleteEvent => todo!(),
                        EventType::ForkEvent => todo!(),
                        EventType::GollumEvent => todo!(),
                        EventType::IssueCommentEvent => {}
                        EventType::IssuesEvent => {
                            let action = event.payload.get("action").unwrap().as_str().unwrap();
                            let action = action
                                .chars()
                                .next()
                                .map(|c| {
                                    c.to_uppercase().collect::<String>() + &action[c.len_utf8()..]
                                })
                                .unwrap_or_default();
                            let repo_name = event.repo.name;
                            println!("{}", format!("- {action} an issue in {repo_name}"));
                        }
                        EventType::MemberEvent => todo!(),
                        EventType::PublicEvent => todo!(),
                        EventType::PullRequestEvent => {}
                        EventType::PullRequestReviewEvent => todo!(),
                        EventType::PullRequestReviewCommentEvent => todo!(),
                        EventType::PullRequestReviewThreadEvent => todo!(),
                        EventType::PushEvent => {
                            let commit_count = event.payload.get("size").unwrap();
                            let repo_name = event.repo.name;
                            println!(
                                "{}",
                                format!("- Pushed {commit_count} commits to {repo_name}")
                            );
                        }
                        EventType::ReleaseEvent => todo!(),
                        EventType::SponsorshipEvent => todo!(),
                        EventType::WatchEvent => {}
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
