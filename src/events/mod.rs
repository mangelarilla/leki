use std::str::FromStr;
use chrono::{Datelike, DateTime, Timelike, Utc};
use serenity::all::{ChannelId, ComponentInteractionData, ComponentInteractionDataKind};
use crate::prelude::*;

pub mod trials;
pub mod models;
pub mod signup;
pub mod parse;
pub mod generic;
pub(crate) mod pvp;
pub(crate) mod components;
pub(crate) mod embeds;

pub fn get_date_time(data: &ComponentInteractionData) -> Option<(ChannelId, DateTime<Utc>)> {
    if let ComponentInteractionDataKind::StringSelect { values} = &data.kind {
        let time = values.first().unwrap();
        let (_, channel_day) = data.custom_id.split_once("__").unwrap();
        let (channel, day) = channel_day.split_once("_").unwrap();
        let dt = calculate_next_date(&day)
            // hack for spanish timezone
            .with_hour((&time[..2]).parse::<u32>().unwrap() - 1).unwrap()
            .with_minute((&time[3..]).parse::<u32>().unwrap()).unwrap();
        let id = ChannelId::from_str(&channel).unwrap();
        Some((id, dt))
    } else { None }
}

fn calculate_next_date(day: &str) -> DateTime<Utc> {
    let now = Utc::now();
    let now_diff_monday = now.weekday().num_days_from_monday();
    let target_diff_monday = to_weekday(day).unwrap().num_days_from_monday();
    let next_target = if target_diff_monday > now_diff_monday {
        target_diff_monday - now_diff_monday
    } else if target_diff_monday == now_diff_monday {
        0
    } else {
        target_diff_monday + (7 - now_diff_monday)
    };
    now + chrono::Duration::days(next_target.into())
}