use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Mention, Timestamp};
use crate::events::models::{EventBasicData, EventRole, Player};
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
                Player::Class(user, class, flex) => format!("└{class} <@{user}> {}", format_flex(flex))
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_flex(roles: &Vec<EventRole>) -> String {
    if roles.is_empty() {
        String::new()
    } else {
        let role_strings = roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();
        format!("(Flex: {})", role_strings.join(","))
    }
}

pub(crate) fn basic(title: &str, description: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
}