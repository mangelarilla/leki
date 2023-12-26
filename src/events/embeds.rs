use serenity::builder::CreateEmbed;
use crate::events::models::EventRole;
use crate::events::signup::EventSignupRoles;
use crate::prelude::embeds::*;

pub fn new_event_embed() -> CreateEmbed {
    basic("Nuevo evento", "Elige tipo de evento")
}

pub(super) fn format_with_role(embed: CreateEmbed, data: &impl EventSignupRoles, role: EventRole, label: &str) -> CreateEmbed {
    let signups = data.role(role);

    let formatted_label = if let Some(max) = signups.max() {
        format!("{} ({}/{})", label, signups.len(), max)
    } else {
        format!("{} ({})", label, signups.len())
    };

    embed.field(formatted_label, format_players_embed(&signups.clone().into()), false)
}