pub mod components;
pub mod embeds;

use std::str::FromStr;
use chrono::{Datelike, DateTime, Timelike, Utc, Weekday};
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::ActionRowComponent::InputText;
use serenity::all::{ActionRow, ChannelId, ComponentInteraction, ComponentInteractionData, ComponentInteractionDataKind, UserId};
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

pub fn get_selected_channels(interaction: &ComponentInteraction) -> Vec<ChannelId> {
    if let ComponentInteractionDataKind::ChannelSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
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

pub fn get_date_time(data: &ComponentInteractionData) -> Option<(ChannelId, DateTime<Utc>)> {
    if let ComponentInteractionDataKind::StringSelect { values} = &data.kind {
        let time = values.first().unwrap();
        let (_, channel_day) = data.custom_id.split_once("__").unwrap();
        let (channel, day) = channel_day.split_once("_").unwrap();
        let hour = (&time[..2]).parse::<u32>().unwrap() - 1; // hack for spanish timezone
        let minute = (&time[3..]).parse::<u32>().unwrap();
        let dt = calculate_next_date(&day, hour, minute)
            .with_hour(hour).unwrap()
            .with_minute(minute).unwrap();
        let id = ChannelId::from_str(&channel).unwrap();
        Some((id, dt))
    } else { None }
}

fn calculate_next_date(day: &str, hour: u32, minute: u32) -> DateTime<Utc> {
    let now = Utc::now();

    let now_diff_monday = now.weekday().num_days_from_monday();
    let target_diff_monday = to_weekday(day).unwrap().num_days_from_monday();
    let next_target = if target_diff_monday > now_diff_monday {
        target_diff_monday - now_diff_monday
    } else if target_diff_monday == now_diff_monday {
        if now.hour() > hour || (now.hour() == hour && now.minute() > minute) { 7 } else { 0 }
    } else {
        target_diff_monday + (7 - now_diff_monday)
    };
    now + chrono::Duration::days(next_target.into())
}