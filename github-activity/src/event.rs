use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct JsonEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub repo: JsonRepo,
    pub payload: JsonPayload,
    pub public: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct JsonPayload {
    #[serde(rename = "ref")]
    pub payload_ref: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct JsonRepo {
    pub id: usize,
    pub name: String,
    pub url: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum EventType {
    CreateEvent,
    PushEvent,
    PullRequestEvent,
    IssueEvent,
    ReleaseEvent,
    StarredEvent,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::CreateEvent => write!(f, "CreateEvent"),
            EventType::PushEvent => write!(f, "PushEvent"),
            EventType::PullRequestEvent => write!(f, "PullRequestEvent"),
            EventType::IssueEvent => write!(f, "IssueEvent"),
            EventType::ReleaseEvent => write!(f, "ReleaseEvent"),
            EventType::StarredEvent => write!(f, "StarredEvent"),
        }
    }
}

impl<T: AsRef<str>> From<T> for EventType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "CreateEvent" => EventType::CreateEvent,
            "PushEvent" => EventType::PushEvent,
            "PullRequestEvent" => EventType::PullRequestEvent,
            "IssueEvent" => EventType::IssueEvent,
            "ReleaseEvent" => EventType::ReleaseEvent,
            "StarredEvent" => EventType::StarredEvent,
            _ => panic!("Unknown event type: {}", s.as_ref()),
        }
    }
}

#[derive(Clone)]
pub struct Event {
    pub id: String,
    pub event_type: EventType,
    pub repo: String,
}

impl From<&JsonEvent> for Event {
    fn from(event: &JsonEvent) -> Self {
        let event = event.clone();
        Event {
            id: event.id,
            event_type: event.event_type.into(),
            repo: event.repo.name,
        }
    }
}

pub fn get_events(username: &str) -> Result<Vec<JsonEvent>, Error> {
    let url = format!("https://api.github.com/users/{}/events", username);
    let mut response = ureq::get(&url).call()?;
    let body = response.body_mut().read_to_string()?;
    Ok(serde_json::from_str(&body)?)
}

#[derive(Default)]
pub struct ControledEvents {
    pub create: Vec<Event>,
    pub push: Vec<Event>,
    pub pr: Vec<Event>,
    pub issue: Vec<Event>,
    pub release: Vec<Event>,
    pub star: Vec<Event>,
}

impl From<&[Event]> for ControledEvents {
    fn from(events: &[Event]) -> ControledEvents {
        let mut controled_events = ControledEvents::default();
        events.iter().for_each(|event| match event.event_type {
            EventType::CreateEvent => controled_events.create.push(event.clone()),
            EventType::PushEvent => controled_events.push.push(event.clone()),
            EventType::PullRequestEvent => controled_events.pr.push(event.clone()),
            EventType::IssueEvent => controled_events.issue.push(event.clone()),
            EventType::ReleaseEvent => controled_events.release.push(event.clone()),
            EventType::StarredEvent => controled_events.star.push(event.clone()),
        });
        controled_events
    }
}

impl ControledEvents {
    fn write(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        action: &str,
        events: &[Event],
    ) -> std::fmt::Result {
        match events.len() {
            0 => Ok(()),
            1 => writeln!(f, "{} repo: {}", action, events[0].repo),
            _ => writeln!(
                f,
                "{} {} repos: {}",
                action,
                events.len(),
                events
                    .iter()
                    .map(|e| e.repo.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl std::fmt::Display for ControledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f, "Created", &self.create)?;
        self.write(f, "Pushed to", &self.push)?;
        self.write(f, "Created PR in", &self.pr)?;
        self.write(f, "Created issue in", &self.issue)?;
        self.write(f, "Created release in", &self.release)?;
        self.write(f, "Starred", &self.star)?;
        Ok(())
    }
}
