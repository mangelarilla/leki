use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::UserId;
use crate::events::models::{EventBasicData};

#[derive(Debug)]
pub struct EventGenericData {
    pub title: String,
    pub description: Option<String>,
    pub datetime: Option<DateTime<Utc>>,
    pub duration: DurationString,
    pub leader: String,
    pub signed: Vec<UserId>,
    pub reserves: Vec<UserId>,
    pub absents: Vec<UserId>,
}

impl EventBasicData for EventGenericData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> Option<String> {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> String {self.leader.clone()}
}