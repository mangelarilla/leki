use std::str::FromStr;
use chrono::{Datelike, DateTime, Timelike, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, ButtonStyle, ChannelId, ChannelType, Colour, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInputText, CreateInteractionResponseMessage, CreateModal, CreateSelectMenu, CreateSelectMenuKind, InputTextStyle, Mention, ReactionType, Timestamp, UserId};
use serenity::all::ActionRowComponent::InputText;
use crate::error::Error;
use crate::events::models::{EventBasicData};
use crate::events::signup::EventBackupRoles;
use crate::prelude::*;

pub mod trials;
pub mod models;
pub mod signup;
pub mod parse;
pub mod generic;

pub fn new() -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .embed(CreateEmbed::new().title("Nuevo evento").description("Elige tipo de evento"))
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("create_trial").label("Trial").style(ButtonStyle::Secondary),
            CreateButton::new("create_pvp").label("PvP").style(ButtonStyle::Secondary),
            CreateButton::new("create_generic").label("Generico").style(ButtonStyle::Secondary)
        ])])
        .ephemeral(true)
}

pub fn data(id: &str) -> CreateModal {
    CreateModal::new(id, "Información del evento")
        .components(vec![
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Titulo del evento", "event_title")
                .placeholder("Aventuras en poletas")
                .required(true)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Duracion", "event_duration")
                .placeholder("1h")
                .required(true)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, "Descripción", "trial_description")
                .placeholder("Se empezara a montar 10 minutos antes\nbla bla bla")
                .required(true)),
        ])
}

pub fn select_date(id: &str, components: &Vec<ActionRow>, leader: &UserId) -> Result<CreateInteractionResponseMessage> {
    let title = get_text(components, 0);
    let duration = get_text(components, 1).parse::<DurationString>()
        .map_err(|e| Error::DurationParse(anyhow::Error::msg(e)))?;
    let description = get_text(components, 2);

    Ok(CreateInteractionResponseMessage::new()
        .embed(preview_embed(&title, &description, duration, leader))
        .components(vec![CreateActionRow::SelectMenu(select_days(id))]))
}

pub fn select_time(id: &str, selected_days: &Vec<(ChannelId, String)>) -> CreateModal {
    CreateModal::new(id, "Horas del evento")
        .components(selected_days.into_iter().map(|(channel, day)| {
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, format!("{} - {}", channel, day), day)
                .placeholder("18:00")
                .required(true))
        }).collect())
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
                let (id, day) = input.custom_id.split_once('-').unwrap();
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
        now_diff_monday + target_diff_monday + 1
    };
    now + chrono::Duration::days(next_target.into())
}

fn select_days(id: &str) -> CreateSelectMenu {
    CreateSelectMenu::new(id, CreateSelectMenuKind::Channel {
        channel_types: Some(vec![ChannelType::Text]),
        default_channels: None,
    })
        .max_values(5)
        .placeholder("Canales del evento")
}

fn preview_embed_basic(title: &str,
                       description: &str,
                       duration: DurationString,
                       leader: &UserId) -> CreateEmbed {
    CreateEmbed::new()
        .author(CreateEmbedAuthor::new("Previsualizacion"))
        .title(title)
        .description(description)
        .field(":date: Fecha y Hora:", "", true)
        .field(":hourglass_flowing_sand: Duración", duration, true)
        .field(":crown: Lider", Mention::User(*leader).to_string(), true)
        .color(Colour::from_rgb(0, 255, 0))
}

fn preview_embed(
    title: &str,
    description: &str,
    duration: DurationString,
    leader: &UserId
) -> CreateEmbed {
    preview_embed_basic(title, description, duration, leader)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field("Apuntados (0/12)", "", false)
        .field(":wave: Reservas (0)", "", false)
        .field(":x: Ausencias (0)", "", false)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/d/d7/ON-icon-zonestory-assisted.png")
}

fn event_embed_basic(data: &impl EventBasicData) -> CreateEmbed {
    CreateEmbed::new()
        .title(data.title())
        .description(if let Some(description) = data.description() {description} else {"".to_string()})
        .field(":date: Fecha y Hora:", if let Some(datetime) = data.datetime() {
            format!("<t:{}:f>", datetime.timestamp())
        } else {"".to_string()}, true)
        .field(":hourglass_flowing_sand: Duración", data.duration().to_string(), true)
        .field(":crown: Lider", data.leader(), true)
        .timestamp(Timestamp::now())
        .footer(CreateEmbedFooter::new("Ultima modificacion:"))
        .color(Colour::from_rgb(0, 255, 0))
}

fn event_embed_backup(data: &impl EventBackupRoles, embed: CreateEmbed) -> CreateEmbed {
    let reserves = data.reserves();
    let absents = data.absents();
    embed
        .field(format!(":wave: Reservas ({})", reserves.len()), format_players_embed(&reserves), false)
        .field(format!(":x: Ausencias ({})", absents.len()), format_players_embed(&absents), false)
}

fn format_players_embed(players: &Vec<UserId>) -> String {
    players.iter()
        .map(|player| format!("└ <@{player}>"))
        .collect::<Vec<String>>()
        .join("\n")
}

fn event_components_backup() -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        CreateButton::new("signup_reserve")
            .label("Reserva")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("👋".to_string())),
        CreateButton::new("signup_absent")
            .label("Ausencia")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("❌".to_string()))
    ])
}