use serenity::all::{UserId};
use crate::events::models::{EventKind, Player, PlayersInRole};

pub trait EventBackupRoles {
    fn reserves(&self) -> Vec<Player>;
    fn absents(&self) -> Vec<Player>;
    fn add_absent(&mut self, user: UserId);
    fn add_reserve(&mut self, user: UserId);
}

pub trait EventSignupRoles<T> {
    fn is_role_full(&self, role: T) -> bool;
    fn signup(&mut self, role: T, user: UserId);
    fn signup_class(&mut self, role: T, user: UserId, class: String);
    fn role(&self, role: T) -> &PlayersInRole;
}

impl EventBackupRoles for EventKind {
    fn reserves(&self) -> Vec<Player> {
        match self {
            EventKind::Trial(t) => t.reserves(),
            EventKind::Generic(g) => g.reserves(),
            EventKind::PvP(p) => p.reserves(),
        }
    }
    fn absents(&self) -> Vec<Player> {
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