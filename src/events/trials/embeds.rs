use serenity::all::{CreateEmbed};
use crate::events::signup::EventSignupRoles;
use crate::events::trials::models::TrialData;
use crate::events::trials::TrialRole;
use crate::prelude::embeds::*;

pub fn trial_embed(data: &TrialData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("Guias:", data.guides().unwrap_or("".to_string()), false)
        .field("AddOns recomendados:", data.addons().unwrap_or("".to_string()), false)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_trial_role(embed, data, TrialRole::Tank, "<:tank:1154134006036713622> Tanks");
    let embed = format_trial_role(embed, data, TrialRole::DD, "<:dd:1154134731756150974> DD");
    let embed = format_trial_role(embed, data, TrialRole::Healer, "<:healer:1154134924153065544> Healers");

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
}

fn format_trial_role(embed: CreateEmbed, data: &TrialData, role: TrialRole, label: &str) -> CreateEmbed {
    let signups = data.role(role);
    embed.field(
        format!("{} ({}/{})", label, signups.len(), data.max(role)),
        format_players_embed(signups),
        false
    )
}