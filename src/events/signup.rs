use serenity::all::{UserId};
use crate::events::models::{EventKind};

pub trait EventBackupRoles {
    fn reserves(&self) -> Vec<UserId>;
    fn absents(&self) -> Vec<UserId>;
    fn add_absent(&mut self, user: UserId);
    fn add_reserve(&mut self, user: UserId);
}

pub trait EventSignupRoles<T> {
    fn is_role_full(&self, role: T) -> bool;
    fn signup(&mut self, role: T, user: UserId, class: String);
}

impl EventBackupRoles for EventKind {
    fn reserves(&self) -> Vec<UserId> {
        match self {
            EventKind::Trial(t) => t.reserves(),
            EventKind::Generic(g) => g.reserves(),
            EventKind::PvP(p) => p.reserves(),
        }
    }
    fn absents(&self) -> Vec<UserId> {
        match self {
            EventKind::Trial(t) => t.absents(),
            EventKind::Generic(g) => g.absents(),
            EventKind::PvP(p) => p.absents(),
        }
    }
    fn add_absent(&mut self, user: UserId) {
        match self {
            EventKind::Trial(t) => t.add_absent(user),
            EventKind::Generic(g) => g.add_absent(user),
            EventKind::PvP(p) => p.add_absent(user),
        }
    }
    fn add_reserve(&mut self, user: UserId) {
        match self {
            EventKind::Trial(t) => t.add_reserve(user),
            EventKind::Generic(g) => g.add_reserve(user),
            EventKind::PvP(p) => p.add_reserve(user),
        }
    }
}

pub(super) fn signups_no_class(signups: &Vec<(String, UserId)>) -> Vec<UserId> {
    signups.iter().map(|(_,p)| *p).collect()
}