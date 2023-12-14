use serenity::all::UserId;
use crate::events::generic::models::EventGenericData;
use crate::events::models::{EventSignups, remove_from_role};
use crate::events::signup::EventBackupRoles;

impl EventBackupRoles for EventGenericData {
    fn reserves(&self) -> Vec<UserId> {self.reserves.clone()}
    fn absents(&self) -> Vec<UserId> {self.absents.clone()}
    fn add_absent(&mut self, user: UserId) {
        self.remove_signup(user);
        self.absents.push(user)
    }
    fn add_reserve(&mut self, user: UserId) {
        self.remove_signup(user);
        self.reserves.push(user)
    }
}

impl EventSignups for EventGenericData {
    fn signups(&self) -> Vec<UserId> {self.signed.clone()}
    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut self.signed, user);
    }
}

impl EventGenericData {
    pub fn signup(&mut self, user: UserId) {
        self.remove_signup(user);
        self.signed.push(user);
    }
}