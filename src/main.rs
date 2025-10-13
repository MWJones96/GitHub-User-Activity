use reqwest::blocking::Client;

use crate::event::{Event, EventType};

mod event;

fn build_url(username: &str) -> String {
    format!("https://api.github.com/users/{username}/events")
}

fn uppercase_first_letter(action: &str) -> String {
    let action = action
        .chars()
        .next()
        .map(|c| c.to_uppercase().collect::<String>() + &action[c.len_utf8()..])
        .unwrap_or_default();
    action
}

fn main() {
    let client = Client::builder()
        .user_agent("RustClient/1.0")
        .build()
        .unwrap();

    match client.get(build_url("0atman")).send() {
        Ok(res) => {
            let json = res.text().unwrap();
            let events: Vec<Event> = serde_json::from_str(&json).unwrap();
            let events: Vec<Event> = events.into_iter().rev().collect();
            for event in events {
                let repo_name = event.get_repo_name();
                let timestamp = event.get_created_at();
                let message: String = match event.get_event_type() {
                    Some(_type) => match _type {
                        EventType::CommitCommentEvent => {
                            let comment = event.get_payload_data("comment");
                            let commit_id = comment.get("commit_id").unwrap().as_str().unwrap();
                            let comment_body = comment.get("body").unwrap().as_str().unwrap();
                            format!("Commented on commit {}: {comment_body}", &commit_id[..6])
                        }
                        EventType::CreateEvent => {
                            let ref_type = event.get_payload_data("ref_type").as_str().unwrap();
                            if ref_type.eq("repository") {
                                format!("New repository created: {repo_name}")
                            } else {
                                let _ref = event.get_payload_data("ref").as_str().unwrap();
                                format!("New {ref_type} was created in {repo_name}: {_ref}")
                            }
                        }
                        EventType::DeleteEvent => {
                            let ref_type = event.get_payload_data("ref_type").as_str().unwrap();
                            let _ref = event.get_payload_data("ref").as_str().unwrap();
                            format!("Deleted a {ref_type} from {repo_name}: {_ref}")
                        }
                        EventType::ForkEvent => {
                            let forkee = event
                                .get_payload_data("forkee")
                                .get("full_name")
                                .unwrap()
                                .as_str()
                                .unwrap();
                            format!("Forked from repository {repo_name} to {forkee}")
                        }
                        EventType::GollumEvent => {
                            let page_count =
                                event.get_payload_data("pages").as_array().unwrap().len();
                            format!(
                                "Created/updated {page_count} wiki page{}",
                                if page_count > 1 { "s" } else { "" }
                            )
                        }
                        EventType::IssueCommentEvent => {
                            /*
                            let action = uppercase_first_letter(
                                event.get_payload_data("action").as_str().unwrap(),
                            );
                            let issue = event.get_payload_data("issue");
                            let comment = event.get_payload_data("comment");
                            format!("{action} a comment on issue {issue}: {comment}")
                            */
                            format!("")
                        }
                        EventType::IssuesEvent => {
                            let action = event.get_payload_data("action").as_str().unwrap();
                            let action = uppercase_first_letter(action);
                            format!("{action} an issue in {repo_name}")
                        }
                        EventType::MemberEvent => "".to_owned(),
                        EventType::PublicEvent => "".to_owned(),
                        EventType::PullRequestEvent => "".to_owned(),
                        EventType::PullRequestReviewEvent => "".to_owned(),
                        EventType::PullRequestReviewCommentEvent => "".to_owned(),
                        EventType::PullRequestReviewThreadEvent => "".to_owned(),
                        EventType::PushEvent => {
                            let commit_count =
                                event.get_payload_data("distinct_size").as_number().unwrap();
                            format!(
                                "Pushed {commit_count} commit{} to {repo_name}",
                                if commit_count.as_u64().unwrap() == 1 {
                                    ""
                                } else {
                                    "s"
                                },
                            )
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
