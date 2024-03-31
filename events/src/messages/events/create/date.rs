use chrono::{Datelike, DateTime, Timelike, Utc, Weekday};
use serenity::all::{ChannelId, ChannelType, ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Message};
use crate::events::Event;
use crate::prelude::*;

pub(super) async fn select_date(message: &Message, interaction: &ComponentInteraction, ctx: &Context, event: &mut Event) -> Result<(ComponentInteraction, ChannelId)> {

    // Select day
    interaction.create_response(&ctx.http, select_day_channel()).await?;
    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        if let Some(channel) = get_selected_channel(&interaction) {
            let name = channel.name(&ctx.http).await?;
            let day = get_channel_weekday(&name).ok_or(Error::NotDay(name.to_string()))?;

            // Select time
            interaction.create_response(&ctx.http, select_time(&day)).await?;
            if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
                if let Some(time) = get_selected_option(&interaction) {
                    let hour = (&time[..2]).parse::<u32>()?; // hack for spanish timezone
                    let minute = (&time[3..]).parse::<u32>()?;

                    event.datetime = calculate_next_date(&day, hour, minute)
                        .with_hour(hour)
                        .map(|dt| dt.with_minute(minute))
                        .flatten();

                    return Ok((interaction, channel));
                }
            }
        }
    }

    Err(Error::Timeout)
}

fn select_day_channel() -> CreateInteractionResponse {
    let channel_selector = CreateSelectMenuKind::Channel {
        channel_types: Some(vec![ChannelType::Text]),
        default_channels: None,
    };

    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(CreateSelectMenu::new("create_event_day_select", channel_selector)
                .placeholder("Canales del evento"))
    )
}

fn select_time(day: &str) -> CreateInteractionResponse {
    let time_options = CreateSelectMenuKind::String {
        options: vec![
            time_option("11:00"), time_option("12:00"),
            time_option("16:00"), time_option("16:30"),
            time_option("17:00"), time_option("17:30"),
            time_option("18:00"), time_option("18:30"),
            time_option("19:00"), time_option("19:30"),
            time_option("20:00"), time_option("20:30"),
            time_option("21:00"), time_option("21:30"),
            time_option("22:00"), time_option("22:30"),
            time_option("23:00"), time_option("23:30"),
        ]
    };

    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(CreateSelectMenu::new("create_event_time_select", time_options)
                .placeholder(format!("Selecciona hora para el {}", day)))
    )
}

fn get_channel_weekday(channel_name: &str) -> Option<String> {
    let weekdays = vec!["lunes", "martes", "miercoles", "jueves", "viernes", "sabado", "domingo"];
    for weekday in weekdays {
        let channel_no_accents = unidecode::unidecode(channel_name);
        if channel_no_accents.contains(weekday) {
            return Some(weekday.to_string());
        }
    }

    None
}

fn time_option(time: &str) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(time, time)
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
    now + chrono::Duration::try_days(next_target.into()).unwrap()
}

fn to_weekday(day: &str) -> Option<Weekday> {
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
