use duration_string::DurationString;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::{ActionRow, Message, UserId};
use crate::events::generic::models::EventGenericData;
use crate::events::models::{EventKind, Player};
use crate::events::pvp::models::PvPData;
use crate::events::trials::models::TrialData;
use crate::prelude::get_input_value;

impl From<Message> for EventKind {
    fn from(value: Message) -> Self {
        let event_embed = value.embeds.first().unwrap();
        let thumbnail = event_embed.thumbnail.as_ref().unwrap();
        match thumbnail.url.as_str() {
            "https://images.uesp.net/d/d7/ON-icon-zonestory-assisted.png" => EventKind::Generic(EventGenericData::try_from(value).unwrap()),
            "https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png" => EventKind::Trial(TrialData::try_from(value).unwrap()),
            "https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png" => EventKind::PvP(PvPData::try_from(value).unwrap()),
            _ => unreachable!("No other images registered")
        }
    }
}

pub(crate) fn get_max(text: &str) -> String {
    tracing::info!("get_max: {text}");
    lazy_static! {
        static ref RE: Regex = Regex::new(r".+\/(?P<max>\d+)").unwrap();
    }
    RE.captures(text).and_then(|cap| {
        cap.name("max").map(|max| max.as_str().to_string())
    }).unwrap()
}

pub(super) fn parse_basic_from_modal(components: &Vec<ActionRow>) -> (String, String, DurationString) {
    let title = get_input_value(components, 0);
    let duration = get_input_value(components, 1).unwrap().parse::<DurationString>()
        .unwrap();
    let description = get_input_value(components, 2);

    (title.unwrap(), description.unwrap(), duration)
}

pub fn parse_player(text: &str) -> Player {
    tracing::info!("parse_player: {text}");
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<class><:.+>)*\s*<@(?P<player>\d+)>").unwrap();
    }
    RE.captures(text).and_then(|cap| Option::from({
        let class = cap.name("class")
            .map(|max| max.as_str().to_string());
        let user = cap.name("player")
            .map(|max| UserId::new(max.as_str().parse::<u64>().unwrap()))
            .unwrap();
        if let Some(class) = class {
            Player::Class(user, class)
        } else {
            Player::Basic(user)
        }

    })).unwrap()
}

pub fn empty_to_option(text: String) -> Option<String> {
    if text.is_empty() { None } else { Some(text) }
}