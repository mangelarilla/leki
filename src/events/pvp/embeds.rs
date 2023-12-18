use serenity::all::CreateEmbed;
use crate::events::pvp::models::PvPData;
use crate::events::pvp::PvPRole;
use crate::events::signup::EventSignupRoles;
use crate::prelude::embeds::*;

pub fn pvp_embed(data: &PvPData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_pvp_role(embed, data, PvPRole::Tank, "<:tank:1154134006036713622> Tanks");
    let embed = format_pvp_role(embed, data, PvPRole::Brawler, "<:dd:1154134731756150974> Brawlers");
    let embed = format_pvp_role(embed, data, PvPRole::Healer, "<:healer:1154134924153065544> Healers");
    let embed = format_pvp_role(embed, data, PvPRole::Bomber, ":bomb: Bombers");
    let embed = format_pvp_role(embed, data, PvPRole::Ganker, ":knife: Gankers");

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png")
}

fn format_pvp_role(embed: CreateEmbed, data: &PvPData, role: PvPRole, label: &str) -> CreateEmbed {
    let signups = data.role(role);
    let max = data.max(role);

    let formatted_label = if max == 0 {
        format!("{} ({})", label, signups.len())
    } else {
        format!("{} ({}/{})", label, signups.len(), data.max(role))
    };

    embed.field(formatted_label, format_players_embed(signups), false)
}