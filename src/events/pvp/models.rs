use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::UserId;
use crate::events::models::EventBasicData;

#[derive(Debug)]
pub struct PvPData {
    pub title: String,
    pub description: Option<String>,
    pub datetime: Option<DateTime<Utc>>,
    pub duration: DurationString,
    pub leader: String,
    pub tanks: Vec<(String, UserId)>,
    pub brawlers: Vec<(String, UserId)>,
    pub bombers: Vec<(String, UserId)>,
    pub healers: Vec<(String, UserId)>,
    pub max_tanks: usize,
    pub max_healers: usize,
    pub reserves: Vec<UserId>,
    pub absents: Vec<UserId>,
}

impl EventBasicData for PvPData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> Option<String> {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> String {self.leader.clone()}
}