pub mod components;
pub mod embeds;

use chrono::{Weekday};
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::ActionRowComponent::InputText;
use serenity::all::{ActionRow, ComponentInteraction, ComponentInteractionDataKind, UserId};
use tracing::{instrument};

pub fn get_input_value(components: &Vec<ActionRow>, idx: usize) -> Option<String> {
    let input_text = components.get(idx)
        .map(|row| row.components.get(0))
        .flatten().unwrap();

    if let InputText(input) = input_text {
        input.value.clone()
    } else {
        None
    }
}

pub fn get_selected_option(interaction: &ComponentInteraction) -> Option<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        Some(values.first().unwrap().to_string())
    } else { None }
}

pub fn get_selected_options(interaction: &ComponentInteraction) -> Vec<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
}

pub fn get_selected_users(interaction: &ComponentInteraction) -> Vec<UserId> {
    if let ComponentInteractionDataKind::UserSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
}

pub fn all_weekdays() -> Vec<&'static str> {
    vec!["lunes", "martes", "miercoles", "jueves", "viernes", "sabado", "domingo"]
}

pub fn get_channel_weekday(channel_name: &str) -> Option<String> {
    let weekdays = all_weekdays();
    for weekday in weekdays {
        let channel_no_accents = unidecode::unidecode(channel_name);
        if channel_no_accents.contains(weekday) {
            return Some(weekday.to_string());
        }
    }

    None
}

pub fn to_weekday(day: &str) -> Option<Weekday> {
    match day {
        "lunes" => Some(Weekday::Mon),
        "martes"=> Some(Weekday::Tue),
        "miercoles" => Some(Weekday::Wed),
        "jueves" => Some(Weekday::Thu),
        "viernes" => Some(Weekday::Fri),
        "sabado" => Some(Weekday::Sat),
        "domingo" => Some(Weekday::Sun),
        _ => None
    }
}

#[instrument]
pub fn parse_event_link(text: &str) -> (u64, u64, u64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^https:\/\/discord\.com\/channels\/(?P<guild>\d+)\/(?P<channel>\d+)\/(?P<msg>\d+)$").unwrap();
    }

    RE.captures(text.lines().next().unwrap()).and_then(|cap| Option::from({
        (cap.name("guild").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap(),
         cap.name("channel").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap(),
         cap.name("msg").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap()
        )
    })).unwrap()
}