use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub enum EventType {
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

#[derive(Deserialize, Debug)]
pub struct Repo {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    _type: Option<EventType>,
    repo: Repo,
    payload: serde_json::Value,
    created_at: String,
}

impl Event {
    pub fn get_repo_name(&self) -> &str {
        &self.repo.name
    }

    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }

    pub fn get_event_type(&self) -> &Option<EventType> {
        &self._type
    }

    pub fn get_payload_data(&self, key: &str) -> &Value {
        self.payload.get(key).unwrap()
    }
}
