use serenity::all::{CreateEmbed};
use crate::events::embeds::format_with_role;
use crate::events::trials::models::TrialData;
use crate::events::trials::TrialRole;
use crate::prelude::embeds::*;

pub fn trial_embed(data: &TrialData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("Guias:", data.guides().unwrap_or("".to_string()), false)
        .field("AddOns recomendados:", data.addons().unwrap_or("".to_string()), false)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_with_role(embed, data, TrialRole::Tank, "<:tank:1154134006036713622> Tanks");
    let embed = format_with_role(embed, data, TrialRole::DD, "<:dd:1154134731756150974> DD");
    let embed = format_with_role(embed, data, TrialRole::Healer, "<:healer:1154134924153065544> Healers");

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
}

pub(super) fn trial_comp_defaults() -> CreateEmbed {
    CreateEmbed::new()
        .title("Composicion por defecto")
        .field("Tanks", "2", true)
        .field("DD", "8", true)
        .field("Healers", "2", true)
}