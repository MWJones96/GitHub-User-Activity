use reqwest::blocking::Client;

use crate::event::{Event, EventType};

mod event;

fn build_url(username: &str) -> String {
    format!("https://api.github.com/users/{username}/events")
}

fn main() {
    let client = Client::builder()
        .user_agent("RustClient/1.0")
        .build()
        .unwrap();

    match client.get(build_url("mwjones96")).send() {
        Ok(res) => {
            let json = res.text().unwrap();
            let events: Vec<Event> = serde_json::from_str(&json).unwrap();
            let events: Vec<Event> = events.into_iter().rev().collect();
            if events.len() > 0 {
                println!("Output:");
            }
            for event in events {
                let repo_name = event.get_repo_name();
                let timestamp = event.get_created_at();
                let message: String = match event.get_event_type() {
                    Some(_type) => match _type {
                        EventType::CommitCommentEvent => "".to_owned(),
                        EventType::CreateEvent => {
                            let ref_type = event
                                .get_payload_data("ref_type")
                                .unwrap()
                                .as_str()
                                .unwrap();
                            if ref_type.eq("repository") {
                                format!("New repository created: {repo_name}")
                            } else {
                                let _ref = event.get_payload_data("ref").unwrap().as_str().unwrap();
                                format!("New {ref_type} was created in {repo_name}: {_ref}")
                            }
                        }
                        EventType::DeleteEvent => "".to_owned(),
                        EventType::ForkEvent => "".to_owned(),
                        EventType::GollumEvent => "".to_owned(),
                        EventType::IssueCommentEvent => "".to_owned(),
                        EventType::IssuesEvent => {
                            let action =
                                event.get_payload_data("action").unwrap().as_str().unwrap();
                            let action = action
                                .chars()
                                .next()
                                .map(|c| {
                                    c.to_uppercase().collect::<String>() + &action[c.len_utf8()..]
                                })
                                .unwrap_or_default();
                            format!("{action} an issue in {repo_name}")
                        }
                        EventType::MemberEvent => "".to_owned(),
                        EventType::PublicEvent => "".to_owned(),
                        EventType::PullRequestEvent => "".to_owned(),
                        EventType::PullRequestReviewEvent => "".to_owned(),
                        EventType::PullRequestReviewCommentEvent => "".to_owned(),
                        EventType::PullRequestReviewThreadEvent => "".to_owned(),
                        EventType::PushEvent => {
                            let commit_count = event
                                .get_payload_data("distinct_size")
                                .unwrap()
                                .as_number()
                                .unwrap();
                            format!("Pushed {commit_count} commits to {repo_name}")
                        }
                        EventType::ReleaseEvent => "".to_owned(),
                        EventType::SponsorshipEvent => "".to_owned(),
                        EventType::WatchEvent => "".to_owned(),
                    },
                    None => "".to_owned(),
                };
                println!("- {timestamp}: {message}")
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
