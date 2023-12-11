use chrono::{DateTime, Utc};
use duration_string::DurationString;
use lazy_static::lazy_static;
use serenity::all::{Message, UserId};
use regex::Regex;
use crate::prelude::*;

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
    pub max_tanks: u8,
    pub max_dds: u8,
    pub max_healers: u8,
    pub reserves: Vec<UserId>,
    pub absents: Vec<UserId>,
}

pub fn parse_trial_data(preview: &Message) -> Result<TrialData> {
    let trial_embed = preview.embeds.first().unwrap();
    let fields = &trial_embed.fields;
    let datetime = fields.get(0).unwrap().value.clone()
        .replace("<t:", "")
        .replace(":f>", "")
        .parse::<i64>().ok();
    let tanks = fields.get(7).unwrap();
    let dds = fields.get(8).unwrap();
    let healers = fields.get(9).unwrap();
    let reserves = fields.get(10).unwrap();
    let absents = fields.get(11).unwrap();

    Ok(TrialData {
        title: trial_embed.title.clone().unwrap(),
        description: trial_embed.description.clone(),
        datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
        duration: fields.get(1).unwrap().value.parse::<DurationString>().map_err(anyhow::Error::msg)?,
        leader: fields.get(2).unwrap().value.clone(),
        guides: fields.get(3).unwrap().value.clone(),
        addons: fields.get(4).unwrap().value.clone(),
        tanks: tanks.value.clone().lines().map(|s| parse_player_class(s)).collect(),
        max_tanks: get_max(&tanks.name).parse::<u8>()?,
        dds: dds.value.clone().lines().map(|s| parse_player_class(s)).collect(),
        max_dds: get_max(&dds.name).parse::<u8>()?,
        healers: healers.value.clone().lines().map(|s| parse_player_class(s)).collect(),
        max_healers: get_max(&healers.name).parse::<u8>()?,
        reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
        absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
    })
}

fn get_max(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".+\/(?P<max>\d+)").unwrap();
    }
    RE.captures(text).and_then(|cap| {
        cap.name("max").map(|max| max.as_str().to_string())
    }).unwrap()
}

fn parse_player(text: &str) -> UserId {
    let id = text.replace("└", "")
        .replace("<@", "")
        .replace(">", "")
        .trim()
        .parse::<u64>();

    UserId::new(id.unwrap())
}

fn parse_player_class(text: &str) -> (String, UserId) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^└(?P<class><:.+>)\s<@(?P<player>\d+)>").unwrap();
    }
    RE.captures(text).and_then(|cap| Option::from({
        (cap.name("class").map(|max| max.as_str().to_string()).unwrap(),
         cap.name("player").map(|max| UserId::new(max.as_str().parse::<u64>().unwrap())).unwrap())
    })).unwrap()
}