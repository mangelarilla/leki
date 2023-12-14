use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{CreateActionRow, UserId};
use serenity::builder::CreateEmbed;
use crate::events::generic::{event_components, event_embed};
use crate::events::generic::models::EventGenericData;
use crate::events::trials::models::TrialData;
use crate::events::trials::{trial_components, trial_embed};

pub trait EventBasicData {
    fn title(&self) -> String;
    fn description(&self) -> Option<String>;
    fn datetime(&self) -> Option<DateTime<Utc>>;
    fn duration(&self) -> DurationString;
    fn leader(&self) -> String;
}

pub trait EventSignups {
    fn signups(&self) -> Vec<UserId>;
    fn remove_signup(&mut self, user: UserId);
}

pub enum EventKind {
    Trial(TrialData), Generic(EventGenericData)
}

impl EventKind {
    pub fn get_embed(&self) -> CreateEmbed {
        match self {
            EventKind::Trial(t) => trial_embed(t),
            EventKind::Generic(g) => event_embed(g)
        }
    }
    pub fn get_components(&self) -> Vec<CreateActionRow> {
        match self {
            EventKind::Trial(_) => trial_components(),
            EventKind::Generic(_) => event_components("signup_event")
        }
    }

    pub fn set_datetime(&mut self, dt: DateTime<Utc>) {
        match self {
            EventKind::Trial(t) => t.datetime = Some(dt),
            EventKind::Generic(g) => g.datetime = Some(dt)
        }
    }
}

impl EventSignups for EventKind {
    fn signups(&self) -> Vec<UserId> {
        match self {
            EventKind::Trial(trial) => trial.signups(),
            EventKind::Generic(g) => g.signups()
        }
    }
    fn remove_signup(&mut self, user: UserId) {
        match self {
            EventKind::Trial(trial) => trial.remove_signup(user),
            EventKind::Generic(g) => g.remove_signup(user)
        }
    }
}

impl EventBasicData for EventKind {
    fn title(&self) -> String {
        match self {
            EventKind::Trial(t) => t.title(),
            EventKind::Generic(g) => g.title()
        }
    }
    fn description(&self) -> Option<String> {
        match self {
            EventKind::Trial(t) => t.description(),
            EventKind::Generic(g) => g.description()
        }
    }
    fn datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            EventKind::Trial(t) => t.datetime(),
            EventKind::Generic(g) => g.datetime()
        }
    }
    fn duration(&self) -> DurationString {
        match self {
            EventKind::Trial(t) => t.duration(),
            EventKind::Generic(g) => g.duration()
        }
    }
    fn leader(&self) -> String {
        match self {
            EventKind::Trial(t) => t.leader(),
            EventKind::Generic(g) => g.leader()
        }
    }
}

pub(super) fn remove_from_role(list: &mut Vec<UserId>, user: UserId) {
    let index = list.iter().position(|player| *player == user);
    if let Some(index) = index {
        list.remove(index);
    }
}