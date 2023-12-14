use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{UserId};
use crate::events::models::{EventBasicData};

#[derive(Debug)]
pub struct TrialData {
    pub title: String,
    pub description: Option<String>,
    pub datetime: Option<DateTime<Utc>>,
    pub duration: DurationString,
    pub leader: String,
    pub guides: String,
    pub addons: String,
    pub tanks: Vec<(String, UserId)>,
    pub dds: Vec<(String, UserId)>,
    pub healers: Vec<(String, UserId)>,
    pub max_tanks: usize,
    pub max_dds: usize,
    pub max_healers: usize,
    pub reserves: Vec<UserId>,
    pub absents: Vec<UserId>,
}

impl EventBasicData for TrialData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> Option<String> {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> String {self.leader.clone()}
}