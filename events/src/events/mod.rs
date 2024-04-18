use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use duration_string::DurationString;
use rand::prelude::IteratorRandom;
use serenity::all::{Colour, CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Mention, RoleId, ScheduledEventId, Timestamp, UserId};

pub(crate) mod event_role;
pub(crate) mod player;

pub(crate) use event_role::*;
pub(crate) use player::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub title: String,
    pub kind: EventKind,
    pub scope: EventScopes,
    pub description: String,
    pub datetime: Option<DateTime<Utc>>,
    pub duration: DurationString,
    pub leader: UserId,
    pub roles: Vec<PlayersInRole>,
    pub scheduled_event: Option<ScheduledEventId>,
    pub notification_role: Option<RoleId>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    pub fn new(title: String, duration: DurationString, description: String, leader: UserId, kind: EventKind) -> Self {
        Event {
            title, description, duration, leader, kind,
            scope: EventScopes::Public,
            datetime: None,
            scheduled_event: None,
            notification_role: None,
            roles: kind.roles()
                .into_iter()
                .map(|role| PlayersInRole {role, players: vec![], max: kind.default_role_max(role) })
                .collect()
        }
    }

    pub fn set_max(&mut self, role: EventRole, max: Option<usize>) {
        for pr in self.roles.iter_mut() {
            if pr.role == role {
                pr.max = max;
            }
        }
    }

    pub fn clear(&mut self, role: EventRole) {
        for pr in self.roles.iter_mut() {
            if pr.role == role {
                pr.players.clear();
            }
        }
    }

    pub fn add_player(&mut self, role: EventRole, player: Player) -> EventRole {
        let mut add_to_reserve = false;
        for pr in self.roles.iter_mut() {
            if let Some(position) = pr.players.iter().position(|p| p.id == player.id) {
                pr.players.remove(position);
            }

            if pr.role == role {
                if pr.max.is_some_and(|max| max <= pr.players.len()) {
                    add_to_reserve = true;
                } else {
                    pr.players.push(player.clone());
                }
            }
        }

        // prevent recursive mut borrow
        if add_to_reserve {
            let mut player = player.clone();
            player.flex.push(role);
            self.add_player(EventRole::Reserve, player);
            EventRole::Reserve
        } else {
            role
        }
    }

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
                    let reserves = self.roles.iter().find(|pr| pr.role == EventRole::Reserve).unwrap();
                    let reserves_count = reserves.players.iter()
                        .filter(|p| p.flex.contains(role))
                        .count();
                    let max_label = if let Some(max) = pr.max {
                        format!("{}/{max}", pr.players.len())
                    } else {
                        format!("{}", pr.players.len())
                    };

                    let reserves_label = if reserves_count > 0 {
                        format!("+{reserves_count}")
                    } else {
                        "".to_string()
                    };

                    (format!("{} {} ({max_label}) {reserves_label}", pr.role.emoji().to_string(), pr.role),
                     format_players_embed(&pr.players), false)
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
            .image(format!("https://github.com/mangelarilla/leki/blob/main/assets/{}/{}?raw=true",
                           self.kind,
                           random_image(&self.title, self.kind).unwrap().file_name().unwrap().to_string_lossy()))
    }
    pub fn embed_preview(&self) -> CreateEmbed {
        self.embed()
            .author(CreateEmbedAuthor::new("Previsualizacion"))
    }

    pub(crate) async fn image(&self) -> crate::prelude::Result<CreateAttachment> {
        let attachment = CreateAttachment::path(random_image(&self.title, self.kind)?).await?;

        Ok(attachment)
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

    pub fn from_partial_id(id: &str) -> Option<Self> {
        if id.contains("trial") {
            Some(EventKind::Trial)
        } else if id.contains("pvp") {
            Some(EventKind::PvP)
        } else {
            None
        }
    }
}

impl EventScopes {
    pub fn from_partial_id(id: &str) -> Self {
        if id.contains("semi_public") {
            EventScopes::SemiPublic
        } else if id.contains("private") {
            EventScopes::Private
        } else {
            EventScopes::Public
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
        let role_strings = roles.iter().map(|r| r.emoji().to_string()).collect::<Vec<String>>();
        format!("({})", role_strings.join("|"))
    }
}

fn random_image(title: &str, kind: EventKind) -> crate::prelude::Result<PathBuf> {
    let mut path = PathBuf::from("assets");
    path.push(kind.to_string());

    let image = if kind == EventKind::PvP {
        path.read_dir()?
            .filter_map(|f| f.ok())
            .choose(&mut rand::thread_rng())
            .unwrap()
    } else {
        path.read_dir()?
            .filter_map(|f| f.ok())
            .find(|t| t.file_name().to_string_lossy() == guess_image(&title))
            .unwrap()
    };

    Ok(image.path())
}

fn guess_image(title: &str) -> String {
    let title = unidecode::unidecode(&title);
    if title.contains("AA") || title.contains("Aetherian") || title.contains("Aeterico") {
        "aa.jpg".to_string()
    } else if title.contains("AS") || title.contains("Asylum") || title.contains("Amparo") {
        "as.jpg".to_string()
    } else if title.contains("HRC") || title.contains("Hel Ra") || title.contains("Hel-Ra") {
        "hrc.jpg".to_string()
    } else if title.contains("SO") || title.contains("Ophidia") || title.contains("Sanctum") {
        "so.png".to_string()
    } else if title.contains("DSR") || title.contains("Dreadsail") || title.contains("Arrecife") {
        "dsr.jpg".to_string()
    } else if title.contains("SS") || title.contains("Sunspire") || title.contains("Sol") {
        "ss.jpg".to_string()
    } else if title.contains("MoL") || title.contains("Maw") || title.contains("Lorkhaj") {
        "mol.jpg".to_string()
    } else if title.contains("CR") || title.contains("Cloudrest") || title.contains("Nubelia") {
        "cr.jpg".to_string()
    } else if title.contains("SE") || title.contains("Sanity") || title.contains("Locura") {
        "se.jpg".to_string()
    } else if title.contains("HoF") || title.contains("Fabrication") || title.contains("Fabricacion") {
        "hof.jpg".to_string()
    } else if title.contains("KA") || title.contains("Kyne") || title.contains("Egida") {
        "ka.png".to_string()
    } else {
        "generic.jpg".to_string()
    }
}