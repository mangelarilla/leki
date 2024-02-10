use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Mention, ScheduledEventId, Timestamp, UserId};

pub(crate) mod event_role;
pub(crate) mod player;

pub(crate) use event_role::*;
pub(crate) use player::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub kind: EventKind,
    pub scope: EventScopes,
    pub description: String,
    pub datetime: Option<DateTime<Utc>>,
    pub duration: DurationString,
    pub leader: UserId,
    pub roles: Vec<PlayersInRole>,
    pub scheduled_event: Option<ScheduledEventId>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayersInRole {
    pub role: EventRole,
    pub players: Vec<Player>,
    pub max: Option<usize>
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "events.kind", rename_all = "lowercase")]
pub enum EventKind {
    Trial, PvP
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "events.scope", rename_all = "lowercase")]
pub enum EventScopes {
    Public, Private, #[sqlx(rename = "semi-public")] SemiPublic
}

impl Event {
    pub fn embed(&self) -> CreateEmbed {
        CreateEmbed::new()
            .title(&self.title)
            .description(&self.description)
            .field(":date: Fecha y Hora:", if let Some(datetime) = &self.datetime {
                format!("<t:{}:F>", datetime.timestamp())
            } else {"".to_string()}, true)
            .field(":hourglass_flowing_sand: Duración", self.duration.to_string(), true)
            .field(":crown: Lider", Mention::User(self.leader).to_string(), true)
            .fields(self.kind.roles().iter()
                .map(|role| {
                    let pr = self.roles.iter().find(|pr| pr.role == *role).unwrap();
                    let formatted_label = if let Some(max) = pr.max {
                        format!("{} {} ({}/{max})", pr.role.emoji().to_string(), pr.role, pr.players.len())
                    } else {
                        format!("{} {} ({})", pr.role.emoji().to_string(), pr.role, pr.players.len())
                    };

                    (formatted_label, format_players_embed(&pr.players), false)
                })
            )
            .field("", "\u{200b}", false)
            .thumbnail(match self.kind {
                EventKind::Trial => "https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png",
                EventKind::PvP => "https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png"
            })
            .timestamp(Timestamp::now())
            .footer(CreateEmbedFooter::new("Ultima modificacion"))
            .color(Colour::from_rgb(0, 255, 0))
    }
    pub fn embed_preview(&self) -> CreateEmbed {
        self.embed()
            .author(CreateEmbedAuthor::new("Previsualizacion"))
    }
}

impl EventKind {
    pub fn roles(&self) -> Vec<EventRole> {
        match self {
            EventKind::Trial => vec![EventRole::Tank, EventRole::DD, EventRole::Healer, EventRole::Reserve, EventRole::Absent],
            EventKind::PvP => vec![EventRole::Tank, EventRole::Brawler, EventRole::Healer, EventRole::Bomber, EventRole::Ganker, EventRole::Reserve, EventRole::Absent],
        }
    }

    pub fn default_role_max(&self, role: EventRole) -> Option<usize> {
        match self {
            EventKind::Trial => {
                match role {
                    EventRole::Tank => Some(2),
                    EventRole::Healer => Some(2),
                    EventRole::DD => Some(8),
                    _ => None
                }
            }
            EventKind::PvP => None
        }
    }
}

impl Display for EventKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventKind::Trial => write!(f, "trial"),
            EventKind::PvP => write!(f, "pvp"),
        }
    }
}

impl Display for EventScopes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventScopes::Public => write!(f, "public"),
            EventScopes::Private => write!(f, "private"),
            EventScopes::SemiPublic => write!(f, "semi_public"),
        }
    }
}

fn format_players_embed(players: &Vec<Player>) -> String {
    players.iter()
        .map(|player| {
            if let Some(class) = &player.class {
                format!("└ {} {} {}", class.emoji(), player.name, format_flex(&player.flex))
            } else {
                format!("└ {} {}", player.name, format_flex(&player.flex))
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