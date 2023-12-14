use serenity::all::{UserId};
use crate::events::models::{EventSignups, remove_from_role};
use crate::events::signup::EventBackupRoles;
use crate::events::trials::models::TrialData;
use crate::events::trials::TrialRole;

impl EventBackupRoles for TrialData {
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

impl EventSignups for TrialData {
    fn signups(&self) -> Vec<UserId> {
        let mut tanks = signups_no_class(&self.tanks);
        let mut dds = signups_no_class(&self.dds);
        let mut healers = signups_no_class(&self.healers);

        tanks.append(&mut dds);
        tanks.append(&mut healers);
        tanks
    }

    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut signups_no_class(&self.tanks), user);
        remove_from_role(&mut signups_no_class(&self.dds), user);
        remove_from_role(&mut signups_no_class(&self.healers), user);
        remove_from_role(&mut self.reserves, user);
        remove_from_role(&mut self.absents, user);
    }
}

impl TrialData {
    pub fn is_role_full(&self, role: TrialRole) -> bool {
        match role {
            TrialRole::Tank => self.max_tanks == self.tanks.len(),
            TrialRole::DD => self.max_dds == self.dds.len(),
            TrialRole::Healer => self.max_healers == self.healers.len()
        }
    }

    pub fn signup(&mut self, role: TrialRole, user: UserId, class: String) {
        self.remove_signup(user);
        match role {
            TrialRole::Tank => self.tanks.push((class, user)),
            TrialRole::DD => self.dds.push((class, user)),
            TrialRole::Healer => self.healers.push((class, user)),
        }
    }
}

fn signups_no_class(signups: &Vec<(String, UserId)>) -> Vec<UserId> {
    signups.iter().map(|(_,p)| *p).collect()
}