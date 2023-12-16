use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{CreateActionRow, UserId};
use serenity::builder::CreateEmbed;
use crate::events::generic::components::event_generic_signup_components;
use crate::events::generic::embeds::event_generic_embed;
use crate::events::generic::models::EventGenericData;
use crate::events::pvp::components::pvp_signup_components;
use crate::events::pvp::embeds::pvp_embed;
use crate::events::pvp::models::PvPData;
use crate::events::trials::components::trial_signup_components;
use crate::events::trials::embeds::trial_embed;
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

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Player {
    Basic(UserId),
    Class(UserId, String)
}

impl Into<UserId> for Player {
    fn into(self) -> UserId {
        match self {
            Player::Basic(user) => user,
            Player::Class(user, _) => user
        }
    }
}

pub enum EventKind {
    Trial(TrialData), Generic(EventGenericData), PvP(PvPData)
}

impl EventKind {
    pub fn get_embed(&self) -> CreateEmbed {
        match self {
            EventKind::Trial(t) => trial_embed(t, false),
            EventKind::Generic(g) => event_generic_embed(g, false),
            EventKind::PvP(p) => pvp_embed(p, false)
        }
    }
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

pub(super) fn remove_from_role(list: &mut Vec<Player>, user: UserId) {
    let index = list.iter().position(|player| <Player as Into<UserId>>::into(player.clone()) == user);
    if let Some(index) = index {
        list.remove(index);
    }
}