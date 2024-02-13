use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serenity::all::{ButtonStyle, CreateButton, EmojiId, ReactionType};
use strum::{EnumIter, IntoEnumIterator};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Serialize, Deserialize, EnumIter, sqlx::Type)]
#[sqlx(type_name = "events.role", rename_all = "lowercase")]
pub enum EventRole {
    Tank, Healer, Brawler, Bomber, Ganker, DD,
    Reserve, Absent
}

impl EventRole {
    pub fn emoji(&self) -> ReactionType {
        match self {
            EventRole::Tank => ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())},
            EventRole::Healer => ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())},
            EventRole::Brawler => ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())},
            EventRole::Bomber => ReactionType::Unicode("💣".to_string()),
            EventRole::Ganker => ReactionType::Unicode("🔪".to_string()),
            EventRole::DD => ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())},
            EventRole::Reserve => ReactionType::Unicode("👋".to_string()),
            EventRole::Absent => ReactionType::Unicode("❌".to_string()),
        }
    }

    pub fn to_button(&self, id: impl Into<String>, label: impl Into<String>) -> CreateButton {
        CreateButton::new(id)
            .label(label)
            .style(match self {
                EventRole::Reserve | EventRole::Absent => ButtonStyle::Secondary,
                _ => ButtonStyle::Success
            })
            .emoji(self.emoji())
    }

    pub fn to_id(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn from_partial_id(value: impl Into<String>) -> Option<Self> {
        let value = value.into().to_lowercase();
        Self::iter()
            .find(|r| value.contains(&r.to_id()))
    }

    pub fn is_backup_role(&self) -> bool {
        self == &EventRole::Absent || self == &EventRole::Reserve
    }
}

impl Display for EventRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventRole::Tank => "Tanks",
            EventRole::Healer => "Healers",
            EventRole::Brawler => "Brawlers",
            EventRole::Bomber => "Bombers",
            EventRole::Ganker => "Gankers",
            EventRole::DD => "DD",
            EventRole::Reserve => "Reservas",
            EventRole::Absent => "Ausencias",
        }.to_string();
        write!(f, "{}", str)
    }
}

impl FromStr for EventRole {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Tanks" => Ok(EventRole::Tank),
            "Healers" => Ok(EventRole::Healer),
            "Brawlers" => Ok(EventRole::Brawler),
            "Bombers" => Ok(EventRole::Bomber),
            "Gankers" => Ok(EventRole::Ganker),
            "DD" => Ok(EventRole::DD),
            "Reservas" => Ok(EventRole::Reserve),
            "Ausencias" => Ok(EventRole::Absent),
            _ => Err(Error::UnknownRole(s.to_string()))
        }
    }
}