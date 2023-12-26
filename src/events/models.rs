use std::fmt::{Display};
use std::str::FromStr;
use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, Message, UserId};
use serenity::builder::CreateEmbed;
use crate::error::Error;
use crate::events::generic::components::event_generic_signup_components;
use crate::events::generic::models::EventGenericData;
use crate::events::pvp::components::pvp_signup_components;
use crate::events::pvp::models::PvPData;
use crate::events::trials::components::trial_signup_components;
use crate::events::trials::models::TrialData;

pub trait EventBasicData {
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn datetime(&self) -> Option<DateTime<Utc>>;
    fn duration(&self) -> DurationString;
    fn leader(&self) -> UserId;
}

pub trait EventSignups {
    fn signups(&self) -> Vec<Player>;
    fn remove_signup(&mut self, user: UserId);
}

pub trait FromBasicModal {
    fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self;
}

pub trait FromComp {
    fn from_comp_with_preview(components: &Vec<ActionRow>, message: Message) -> Self;
}

pub trait EventEmbed {
    fn get_embed(&self) -> CreateEmbed;
    fn get_embed_preview(&self) -> CreateEmbed;
}

pub trait EventComp {
    fn get_comp_defaults_embed() -> CreateEmbed;
    fn get_comp_new_components() -> Vec<CreateActionRow>;
}

#[derive(Debug, Clone)]
pub struct PlayersInRole {
    players: Vec<Player>,
    max: Option<usize>
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventRole {
    Tank, Healer, Brawler, Bomber, Ganker, DD
}

impl FromStr for EventRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tanks" => Ok(EventRole::Tank),
            "Brawlers" => Ok(EventRole::Brawler),
            "Healers" => Ok(EventRole::Healer),
            "Bombers" => Ok(EventRole::Bomber),
            "Gankers" => Ok(EventRole::Ganker),
            "DD" => Ok(EventRole::DD),
            _ => Err(Error::UnknownRole(s.to_string()))
        }
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
        }.to_string();
        write!(f, "{}", str)
    }
}

impl PlayersInRole {
    pub(crate) fn new(players: Vec<Player>, max: Option<usize>) -> Self {
        PlayersInRole { players, max }
    }
    pub(crate) fn is_role_full(&self) -> bool {
        self.max.is_some_and(|max| max <= self.players.len())
    }

    pub(crate) fn len(&self) -> usize {
        self.players.len()
    }
    pub(crate) fn max(&self) -> Option<usize> {
        self.max
    }
    pub(crate) fn as_slice(&self) -> &[Player] {
        self.players.as_slice()
    }
    pub(crate) fn push(&mut self, player: Player) {
        self.players.push(player)
    }
}

impl Into<Vec<Player>> for PlayersInRole {
    fn into(self) -> Vec<Player> {
        self.players
    }
}

impl Default for PlayersInRole {
    fn default() -> Self {
        PlayersInRole {
            players: vec![], max: None
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Player {
    Basic(UserId),
    Class(UserId, String, Vec<EventRole>)
}

impl Into<UserId> for Player {
    fn into(self) -> UserId {
        match self {
            Player::Basic(user) => user,
            Player::Class(user, _, _) => user
        }
    }
}

#[derive(Debug)]
pub enum EventKind {
    Trial(TrialData), Generic(EventGenericData), PvP(PvPData)
}

impl EventEmbed for EventKind {
    fn get_embed(&self) -> CreateEmbed {
        match self {
            EventKind::Trial(t) => t.get_embed(),
            EventKind::Generic(g) => g.get_embed(),
            EventKind::PvP(p) => p.get_embed()
        }
    }

    fn get_embed_preview(&self) -> CreateEmbed {
        match self {
            EventKind::Trial(t) => t.get_embed_preview(),
            EventKind::Generic(g) => g.get_embed_preview(),
            EventKind::PvP(p) => p.get_embed_preview()
        }
    }
}

impl EventKind {
    pub fn get_components(&self) -> Vec<CreateActionRow> {
        match self {
            EventKind::Trial(_) => trial_signup_components(),
            EventKind::Generic(_) => event_generic_signup_components("signup_generic_event"),
            EventKind::PvP(_) => pvp_signup_components()
        }
    }

    pub fn set_datetime(&mut self, dt: DateTime<Utc>) {
        match self {
            EventKind::Trial(t) => t.datetime = Some(dt),
            EventKind::Generic(g) => g.datetime = Some(dt),
            EventKind::PvP(p) => p.datetime = Some(dt)
        }
    }
}

impl EventSignups for EventKind {
    fn signups(&self) -> Vec<Player> {
        match self {
            EventKind::Trial(trial) => trial.signups(),
            EventKind::Generic(g) => g.signups(),
            EventKind::PvP(p) => p.signups()
        }
    }
    fn remove_signup(&mut self, user: UserId) {
        match self {
            EventKind::Trial(trial) => trial.remove_signup(user),
            EventKind::Generic(g) => g.remove_signup(user),
            EventKind::PvP(p) => p.remove_signup(user),
        }
    }
}

impl EventBasicData for EventKind {
    fn title(&self) -> String {
        match self {
            EventKind::Trial(t) => t.title(),
            EventKind::Generic(g) => g.title(),
            EventKind::PvP(p) => p.title(),
        }
    }
    fn description(&self) -> String {
        match self {
            EventKind::Trial(t) => t.description(),
            EventKind::Generic(g) => g.description(),
            EventKind::PvP(p) => p.description(),
        }
    }
    fn datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            EventKind::Trial(t) => t.datetime(),
            EventKind::Generic(g) => g.datetime(),
            EventKind::PvP(p) => p.datetime(),
        }
    }
    fn duration(&self) -> DurationString {
        match self {
            EventKind::Trial(t) => t.duration(),
            EventKind::Generic(g) => g.duration(),
            EventKind::PvP(p) => p.duration(),
        }
    }
    fn leader(&self) -> UserId {
        match self {
            EventKind::Trial(t) => t.leader(),
            EventKind::Generic(g) => g.leader(),
            EventKind::PvP(p) => p.leader(),
        }
    }
}

pub(super) fn remove_from_role(list: &mut PlayersInRole, user: UserId) {
    let index = list.players.iter().position(|player| <Player as Into<UserId>>::into(player.clone()) == user);
    if let Some(index) = index {
        list.players.remove(index);
    }
}