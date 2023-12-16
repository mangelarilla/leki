use std::str::FromStr;
use chrono::{Datelike, DateTime, Timelike, Utc};
use serenity::all::{ActionRow, ChannelId, CreateModal};
use serenity::all::ActionRowComponent::InputText;
use crate::prelude::*;
use crate::utils::components::short_input;

pub mod trials;
pub mod models;
pub mod signup;
pub mod parse;
pub mod generic;
pub(crate) mod pvp;
pub(crate) mod components;
pub(crate) mod embeds;

pub fn select_time(id: &str, selected_days: &Vec<(ChannelId, String)>) -> CreateModal {
    CreateModal::new(id, "Horas del evento")
        .components(selected_days
            .into_iter()
            .map(|(channel, day)| {
                short_input(day, &format!("{}_{}", channel, day), "18:00", true)
            })
            .collect()
        )
}

pub fn get_date_times(components: &Vec<ActionRow>) -> Vec<(ChannelId, DateTime<Utc>)> {
    get_days_times(components).into_iter()
        .map(|(channel, day, time)| {
            let dt = calculate_next_date(&day)
                // hack for spanish timezone
                .with_hour((&time[..2]).parse::<u32>().unwrap() - 1).unwrap()
                .with_minute((&time[3..]).parse::<u32>().unwrap()).unwrap();
            let id = ChannelId::from_str(&channel).unwrap();
            (id, dt)
        }
        ).collect()
}

fn get_days_times(components: &Vec<ActionRow>) -> Vec<(String, String, String)> {
    components.iter()
        .filter_map(|row| {
            if let InputText(input) = row.components.get(0).unwrap() {
                let (id, day) = input.custom_id.split_once('_').unwrap();
                Some((id.trim().to_string(), day.trim().to_string(), input.value.as_ref().unwrap_or(&"".to_string()).trim().to_string()))
            } else {
                None
            }
        }).collect()
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