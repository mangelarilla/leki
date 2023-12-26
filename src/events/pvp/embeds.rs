use serenity::all::CreateEmbed;
use crate::events::embeds::format_with_role;
use crate::events::models::EventRole;
use crate::events::pvp::models::PvPData;
use crate::prelude::embeds::*;

pub fn pvp_embed(data: &PvPData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_with_role(embed, data, EventRole::Tank, "<:tank:1154134006036713622> Tanks");
    let embed = format_with_role(embed, data, EventRole::Brawler, "<:dd:1154134731756150974> Brawlers");
    let embed = format_with_role(embed, data, EventRole::Healer, "<:healer:1154134924153065544> Healers");
    let embed = format_with_role(embed, data, EventRole::Bomber, ":bomb: Bombers");
    let embed = format_with_role(embed, data, EventRole::Ganker, ":knife: Gankers");

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png")
}

pub(super) fn pvp_comp_defaults() -> CreateEmbed {
    CreateEmbed::new()
        .title("Composicion por defecto")
        .description("No hay maximos por defecto!")
}