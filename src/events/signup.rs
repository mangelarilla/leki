use serenity::all::{UserId};
use crate::events::models::{EventKind};

pub trait EventBackupRoles {
    fn reserves(&self) -> Vec<UserId>;
    fn absents(&self) -> Vec<UserId>;
    fn add_absent(&mut self, user: UserId);
    fn add_reserve(&mut self, user: UserId);
}

impl EventBackupRoles for EventKind {
    fn reserves(&self) -> Vec<UserId> {
        match self {
            EventKind::Trial(t) => t.reserves(),
            EventKind::Generic(g) => g.reserves()
        }
    }
    fn absents(&self) -> Vec<UserId> {
        match self {
            EventKind::Trial(t) => t.absents(),
            EventKind::Generic(g) => g.absents()
        }
    }
    fn add_absent(&mut self, user: UserId) {
        match self {
            EventKind::Trial(t) => t.add_absent(user),
            EventKind::Generic(g) => g.add_absent(user)
        }
    }
    fn add_reserve(&mut self, user: UserId) {
        match self {
            EventKind::Trial(t) => t.add_reserve(user),
            EventKind::Generic(g) => g.add_reserve(user)
        }
    }
}