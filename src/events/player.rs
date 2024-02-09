use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serenity::all::{EmojiId, ReactionType, UserId};
use super::{EventRole};
use crate::prelude::*;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: UserId,
    pub name: String,
    pub class: Option<PlayerClass>,
    pub flex: Vec<EventRole>
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "class", rename_all = "lowercase")]
pub enum PlayerClass {
    Arcanist, Necromancer, Warden, #[sqlx(rename = "dragon-knight")]DragonKnight, Templar, Sorcerer, #[sqlx(rename = "night-blade")]NightBlade
}

impl Player {
    pub fn new(id: UserId, name: String) -> Self {
        Player {id, name, class: None, flex: vec![]}
    }
}

impl PlayerClass {
    pub fn label_es(&self) -> String {
        match self {
            PlayerClass::Arcanist => "Arcanista",
            PlayerClass::Necromancer => "Nigromante",
            PlayerClass::Warden => "Custodio",
            PlayerClass::DragonKnight => "Caballero dragon",
            PlayerClass::Templar => "Templario",
            PlayerClass::Sorcerer => "Brujo",
            PlayerClass::NightBlade => "Hoja de la noche",
        }.to_string()
    }

    pub fn emoji(&self) -> ReactionType {
        let (id, label) = match self {
            PlayerClass::Arcanist => (1154134563392606218, "arcanist"),
            PlayerClass::Necromancer => (1154088177796137030, "necro"),
            PlayerClass::Warden => (1154134387546398720, "warden"),
            PlayerClass::DragonKnight => (1157391862659809280, "dk"),
            PlayerClass::Templar => (1157391868850618388, "templar"),
            PlayerClass::Sorcerer => (1157391866971566100, "sorc"),
            PlayerClass::NightBlade => (1157391864954093608, "nb"),
        };

        ReactionType::Custom { animated: false, id: EmojiId::new(id), name: Some(label.to_string())}
    }
}

impl Display for PlayerClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerClass::Arcanist => write!(f, "Arcanist"),
            PlayerClass::Necromancer => write!(f, "Necromancer"),
            PlayerClass::Warden => write!(f, "Warden"),
            PlayerClass::DragonKnight => write!(f, "DragonKnight"),
            PlayerClass::Templar => write!(f, "Templar"),
            PlayerClass::Sorcerer => write!(f, "Sorcerer"),
            PlayerClass::NightBlade => write!(f, "NightBlade"),
        }
    }
}

impl FromStr for PlayerClass {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Arcanist" => Ok(PlayerClass::Arcanist),
            "Necromancer" => Ok(PlayerClass::Necromancer),
            "Warden" => Ok(PlayerClass::Warden),
            "DragonKnight" => Ok(PlayerClass::DragonKnight),
            "Templar" => Ok(PlayerClass::Templar),
            "Sorcerer" => Ok(PlayerClass::Sorcerer),
            "NightBlade" => Ok(PlayerClass::NightBlade),
            _ => Err(Error::UnknownClass(s.to_string()))
        }
    }
}

