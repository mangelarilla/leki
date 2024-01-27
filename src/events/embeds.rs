use serenity::builder::CreateEmbed;
use crate::events::{EventData, EventSignedRole};
use crate::prelude::embeds::*;

pub(super) fn format_with_role(embed: CreateEmbed, data: &impl EventData, role: EventSignedRole) -> CreateEmbed {
    let signups = data.role(role);

    let formatted_label = if let Some(max) = signups.max() {
        format!("{} {role} ({}/{max})", role.emoji().to_string(), signups.len())
    } else {
        format!("{} {role} ({})", role.emoji().to_string(), signups.len())
    };

    embed.field(formatted_label, format_players_embed(&signups.clone().into()), false)
}