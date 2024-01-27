use std::fmt::Display;
use std::str::FromStr;
use serenity::all::{ButtonStyle, CreateButton, EmojiId, ReactionType};
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventRole {
    Signed(EventSignedRole),
    Reserve, Absent
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventSignedRole {
    Tank, Healer, Brawler, Bomber, Ganker, DD, Participant
}

impl EventSignedRole {
    pub fn emoji(&self) -> ReactionType {
        match self {
            EventSignedRole::Tank => ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())},
            EventSignedRole::Healer => ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())},
            EventSignedRole::Brawler => ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())},
            EventSignedRole::Bomber => ReactionType::Unicode("💣".to_string()),
            EventSignedRole::Ganker => ReactionType::Unicode("🔪".to_string()),
            EventSignedRole::DD => ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())},
            EventSignedRole::Participant => ReactionType::Unicode("✅".to_string())
        }
    }
    pub fn to_button(&self, id: &str) -> CreateButton {
        CreateButton::new(id)
            .label(self.to_string())
            .style(ButtonStyle::Success)
            .emoji(self.emoji())
    }

    pub fn to_id(&self) -> String {
        self.to_string().to_lowercase()
    }
}

impl EventRole {
    pub fn emoji(&self) -> ReactionType {
        match self {
            EventRole::Reserve => ReactionType::Unicode("👋".to_string()),
            EventRole::Absent => ReactionType::Unicode("❌".to_string()),
            EventRole::Signed(s) => s.emoji()
        }
    }
    pub fn to_button(&self, id: &str) -> CreateButton {
        if let EventRole::Signed(s) = self {
            s.to_button(id)
        } else {
            CreateButton::new(id)
                .label(self.to_string())
                .style(ButtonStyle::Secondary)
                .emoji(self.emoji())
        }
    }

    pub fn to_id(&self) -> String {
        self.to_string().to_lowercase()
    }
}

impl FromStr for EventSignedRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tanks" => Ok(EventSignedRole::Tank),
            "Brawlers" => Ok(EventSignedRole::Brawler),
            "Healers" => Ok(EventSignedRole::Healer),
            "Bombers" => Ok(EventSignedRole::Bomber),
            "Gankers" => Ok(EventSignedRole::Ganker),
            "Participantes" => Ok(EventSignedRole::Participant),
            "DD" => Ok(EventSignedRole::DD),
            _ => Err(Error::UnknownRole(s.to_string()))
        }
    }
}

impl FromStr for EventRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match EventSignedRole::from_str(s) {
            Ok(s) => Ok(EventRole::Signed(s)),
            Err(_) => match s {
                "Reservas" => Ok(EventRole::Reserve),
                "Ausencias" => Ok(EventRole::Absent),
                _ => Err(Error::UnknownRole(s.to_string()))
            }
        }
    }
}

impl Display for EventSignedRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventSignedRole::Tank => "Tanks",
            EventSignedRole::Healer => "Healers",
            EventSignedRole::Brawler => "Brawlers",
            EventSignedRole::Bomber => "Bombers",
            EventSignedRole::Ganker => "Gankers",
            EventSignedRole::DD => "DD",
            EventSignedRole::Participant => "Participantes"
        }.to_string();
        write!(f, "{}", str)
    }
}

impl Display for EventRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventRole::Reserve => "Reservas".to_string(),
            EventRole::Absent => "Ausencias".to_string(),
            EventRole::Signed(r) => r.to_string()
        }.to_string();
        write!(f, "{}", str)
    }
}