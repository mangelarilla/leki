use serenity::all::CreateEmbed;
use crate::events::generic::models::EventGenericData;
use crate::events::models::EventSignups;
use crate::prelude::embeds::*;

pub fn event_generic_embed(data: &EventGenericData, is_preview: bool) -> CreateEmbed {
    let signups = data.signups();
    let embed = event_embed_basic(data, is_preview)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field(format!("Apuntados ({})", signups.len()), format_players_embed(&signups), false);
    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/d/d7/ON-icon-zonestory-assisted.png")
}