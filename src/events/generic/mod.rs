use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed};
use crate::events::{event_components_backup, event_embed_backup, event_embed_basic};
use crate::events::generic::models::EventGenericData;

pub mod models;
pub mod signup;

pub fn event_embed(
    data: &EventGenericData,
) -> CreateEmbed {
    let embed = event_embed_basic(data)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field(
            format!("Apuntados ({})", &data.signed.len()),
            &data.signed.iter().map(|player| format!("└<@{player}>")).collect::<Vec<String>>().join("\n"),
            false);
    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/d/d7/ON-icon-zonestory-assisted.png")
}

pub fn event_components(id: &str) -> Vec<CreateActionRow> {
    let class_row = CreateActionRow::Buttons(vec![
        CreateButton::new(id)
            .label("Apuntarse")
            .style(ButtonStyle::Success),
    ]);

    vec![class_row, event_components_backup()]
}