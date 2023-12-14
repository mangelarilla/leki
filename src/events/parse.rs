use chrono::DateTime;
use duration_string::DurationString;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::{Message, UserId};
use crate::events::generic::models::EventGenericData;
use crate::events::models::{EventKind};
use crate::events::pvp::models::PvPData;
use crate::events::trials::models::TrialData;
use crate::prelude::*;

pub trait ParseEventData {
    fn parse_generic(&self) -> Result<EventGenericData>;
    fn parse_trial(&self) -> Result<TrialData>;
    fn parse_pvp(&self) -> Result<PvPData>;
    fn parse_event(&self) -> Option<EventKind>;
}

impl ParseEventData for Message {
    fn parse_generic(&self) -> Result<EventGenericData> {
        let event_embed = self.embeds.first().unwrap();
        let fields = &event_embed.fields;
        let datetime = fields.get(0).unwrap().value.clone()
            .replace("<t:", "")
            .replace(":f>", "")
            .parse::<i64>().ok();
        let signed = fields.get(5).unwrap();
        let reserves = fields.get(6).unwrap();
        let absents = fields.get(7).unwrap();

        Ok(EventGenericData {
            title: event_embed.title.clone().unwrap(),
            description: event_embed.description.clone(),
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
            duration: fields.get(1).unwrap().value.parse::<DurationString>().map_err(anyhow::Error::msg)?,
            leader: fields.get(2).unwrap().value.clone(),
            signed: signed.value.clone().lines().map(|s| parse_player(s)).collect(),
            reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
            absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
        })
    }
    fn parse_trial(&self) -> Result<TrialData> {
        let trial_embed = self.embeds.first().unwrap();
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
            max_tanks: get_max(&tanks.name).parse::<usize>()?,
            dds: dds.value.clone().lines().map(|s| parse_player_class(s)).collect(),
            max_dds: get_max(&dds.name).parse::<usize>()?,
            healers: healers.value.clone().lines().map(|s| parse_player_class(s)).collect(),
            max_healers: get_max(&healers.name).parse::<usize>()?,
            reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
            absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
        })
    }

    fn parse_pvp(&self) -> Result<PvPData> {
        let trial_embed = self.embeds.first().unwrap();
        let fields = &trial_embed.fields;
        let datetime = fields.get(0).unwrap().value.clone()
            .replace("<t:", "")
            .replace(":f>", "")
            .parse::<i64>().ok();
        let tanks = fields.get(7).unwrap();
        let brawlers = fields.get(8).unwrap();
        let healers = fields.get(9).unwrap();
        let bombers = fields.get(10).unwrap();
        let reserves = fields.get(11).unwrap();
        let absents = fields.get(12).unwrap();

        Ok(PvPData {
            title: trial_embed.title.clone().unwrap(),
            description: trial_embed.description.clone(),
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
            duration: fields.get(1).unwrap().value.parse::<DurationString>().map_err(anyhow::Error::msg)?,
            leader: fields.get(2).unwrap().value.clone(),
            tanks: tanks.value.clone().lines().map(|s| parse_player_class(s)).collect(),
            max_tanks: get_max(&tanks.name).parse::<usize>()?,
            brawlers: brawlers.value.clone().lines().map(|s| parse_player_class(s)).collect(),
            bombers: bombers.value.clone().lines().map(|s| parse_player_class(s)).collect(),
            healers: healers.value.clone().lines().map(|s| parse_player_class(s)).collect(),
            max_healers: get_max(&healers.name).parse::<usize>()?,
            reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
            absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
        })
    }

    fn parse_event(&self) -> Option<EventKind> {
        let event_embed = self.embeds.first().unwrap();
        let thumbnail = event_embed.thumbnail.as_ref().unwrap();
        match thumbnail.url.as_str() {
            "https://images.uesp.net/d/d7/ON-icon-zonestory-assisted.png" => self.parse_generic().ok().map(|ev| EventKind::Generic(ev)),
            "https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png" => self.parse_trial().ok().map(|ev| EventKind::Trial(ev)),
            "https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png" => self.parse_pvp().ok().map(|ev| EventKind::PvP(ev)),
            _ => None
        }
    }
}

fn get_max(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".+\/(?P<max>\d+)").unwrap();
    }
    RE.captures(text).and_then(|cap| {
        cap.name("max").map(|max| max.as_str().to_string())
    }).unwrap()
}

pub fn parse_player(text: &str) -> UserId {
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