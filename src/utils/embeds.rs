use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Mention, Timestamp};
use crate::events::models::{EventBasicData, Player};
use crate::events::signup::EventBackupRoles;

pub(crate) fn event_embed_basic(data: &impl EventBasicData, is_preview: bool) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .title(data.title())
        .description(data.description())
        .field(":date: Fecha y Hora:", if let Some(datetime) = data.datetime() {
            format!("<t:{}:F>", datetime.timestamp())
        } else {"".to_string()}, true)
        .field(":hourglass_flowing_sand: Duración", data.duration().to_string(), true)
        .field(":crown: Lider", Mention::User(data.leader()).to_string(), true)
        .timestamp(Timestamp::now())
        .footer(CreateEmbedFooter::new("Ultima modificacion:"))
        .color(Colour::from_rgb(0, 255, 0));

    if is_preview {
        embed.author(CreateEmbedAuthor::new("Previsualizacion"))
    } else {
        embed
    }
}

pub(crate) fn event_embed_backup(data: &impl EventBackupRoles, embed: CreateEmbed) -> CreateEmbed {
    let reserves = data.reserves();
    let absents = data.absents();
    embed
        .field(format!(":wave: Reservas ({})", reserves.len()), format_players_embed(&reserves), false)
        .field(format!(":x: Ausencias ({})", absents.len()), format_players_embed(&absents), false)
}

pub(crate) fn format_players_embed(players: &Vec<Player>) -> String {
    players.iter()
        .map(|player| {
            match player {
                Player::Basic(user) => format!("└ <@{user}>"),
                Player::Class(user, class) => format!("└{class} <@{user}>")
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub(crate) fn basic(title: &str, description: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
}