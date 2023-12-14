use serenity::all::{UserId};
use crate::events::models::{EventSignups, remove_from_role};
use crate::events::pvp::models::PvPData;
use crate::events::pvp::PvPRole;
use crate::events::signup::{EventBackupRoles, EventSignupRoles, signups_no_class};

impl EventBackupRoles for PvPData {
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

impl EventSignups for PvPData {
    fn signups(&self) -> Vec<UserId> {
        let mut tanks = signups_no_class(&self.tanks);
        let mut brawlers = signups_no_class(&self.brawlers);
        let mut healers = signups_no_class(&self.healers);
        let mut bombers = signups_no_class(&self.bombers);

        tanks.append(&mut brawlers);
        tanks.append(&mut healers);
        tanks.append(&mut bombers);
        tanks
    }

    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut signups_no_class(&self.tanks), user);
        remove_from_role(&mut signups_no_class(&self.brawlers), user);
        remove_from_role(&mut signups_no_class(&self.healers), user);
        remove_from_role(&mut signups_no_class(&self.bombers), user);
        remove_from_role(&mut self.reserves, user);
        remove_from_role(&mut self.absents, user);
    }
}

impl EventSignupRoles<PvPRole> for PvPData {
    fn is_role_full(&self, role: PvPRole) -> bool {
        match role {
            PvPRole::Tank => self.max_tanks == self.tanks.len(),
            PvPRole::Healer => self.max_healers == self.healers.len(),
            _ => false,
        }
    }

    fn signup(&mut self, role: PvPRole, user: UserId, class: String) {
        self.remove_signup(user);
        match role {
            PvPRole::Tank => self.tanks.push((class, user)),
            PvPRole::Healer => self.healers.push((class, user)),
            PvPRole::Brawler => self.brawlers.push((class, user)),
            PvPRole::Bomber => self.bombers.push((class, user)),
        }
    }
}

