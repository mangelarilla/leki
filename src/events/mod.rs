use std::str::FromStr;
use chrono::{Datelike, DateTime, Timelike, Utc};
use serenity::all::{ChannelId, ComponentInteractionData, ComponentInteractionDataKind};
use crate::prelude::*;

pub mod trials;
pub mod parse;
pub mod generic;
pub(crate) mod pvp;
pub(crate) mod components;
pub(crate) mod embeds;
pub(crate) mod event_data;
pub(crate) mod event_role;
pub(crate) mod player;

pub(crate) use event_data::EventData;
pub(crate) use event_role::*;
pub(crate) use player::*;

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