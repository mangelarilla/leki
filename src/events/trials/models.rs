use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, Message, UserId};
use crate::events::models::{EventBasicData, EventSignups, Player, remove_from_role};
use crate::events::parse::{empty_to_option, get_max, parse_basic_from_modal, parse_player};
use crate::events::signup::{EventBackupRoles, EventSignupRoles};
use crate::events::trials::TrialRole;
use crate::prelude::get_input_value;

#[derive(Debug)]
pub struct TrialData {
    title: String,
    description: String,
    pub(crate) datetime: Option<DateTime<Utc>>,
    duration: DurationString,
    leader: UserId,
    guides: Option<String>,
    addons: Option<String>,
    tanks: Vec<Player>,
    dds: Vec<Player>,
    healers: Vec<Player>,
    max_tanks: usize,
    max_dds: usize,
    max_healers: usize,
    reserves: Vec<Player>,
    absents: Vec<Player>,
}

impl TrialData {
    pub fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self {
        let (title, description, duration) = parse_basic_from_modal(components);
        let addons = get_input_value(components, 3);
        let guides = get_input_value(components, 4);

        TrialData {
            title,
            description,
            datetime: None,
            duration,
            leader,
            guides,
            addons,
            tanks: vec![],
            dds: vec![],
            healers: vec![],
            max_tanks: 2,
            max_dds: 8,
            max_healers: 2,
            reserves: vec![],
            absents: vec![],
        }
    }

    pub fn guides(&self) -> Option<String> {self.guides.clone()}
    pub fn addons(&self) -> Option<String> {self.addons.clone()}
}

impl EventBasicData for TrialData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> String {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> UserId {self.leader.clone()}
}

impl EventBackupRoles for TrialData {
    fn reserves(&self) -> Vec<Player> {self.reserves.clone()}
    fn absents(&self) -> Vec<Player> {self.absents.clone()}
    fn add_absent(&mut self, user: UserId) {
        self.remove_signup(user);
        self.absents.push(Player::Basic(user))
    }
    fn add_reserve(&mut self, user: UserId) {
        self.remove_signup(user);
        self.reserves.push(Player::Basic(user))
    }
}

impl EventSignups for TrialData {
    fn signups(&self) -> Vec<Player> {
        [self.tanks.as_slice(), self.dds.as_slice(), self.healers.as_slice()].concat()
    }

    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut self.tanks, user);
        remove_from_role(&mut self.dds, user);
        remove_from_role(&mut self.healers, user);
        remove_from_role(&mut self.reserves, user);
        remove_from_role(&mut self.absents, user);
    }
}

impl EventSignupRoles<TrialRole> for TrialData {
    fn is_role_full(&self, role: TrialRole) -> bool {
        match role {
            TrialRole::Tank => self.max_tanks == self.tanks.len(),
            TrialRole::DD => self.max_dds == self.dds.len(),
            TrialRole::Healer => self.max_healers == self.healers.len()
        }
    }

    fn signup(&mut self, role: TrialRole, user: UserId) {
        self.remove_signup(user);
        match role {
            TrialRole::Tank => self.tanks.push(Player::Basic(user)),
            TrialRole::DD => self.dds.push(Player::Basic(user)),
            TrialRole::Healer => self.healers.push(Player::Basic(user)),
        }
    }

    fn signup_class(&mut self, role: TrialRole, user: UserId, class: String) {
        self.remove_signup(user);
        match role {
            TrialRole::Tank => self.tanks.push(Player::Class(user, class)),
            TrialRole::DD => self.dds.push(Player::Class(user, class)),
            TrialRole::Healer => self.healers.push(Player::Class(user, class)),
        }
    }

    fn role(&self, role: TrialRole) -> &Vec<Player> {
        match role {
            TrialRole::Tank => &self.tanks,
            TrialRole::DD => &self.dds,
            TrialRole::Healer => &self.healers
        }
    }

    fn max(&self, role: TrialRole) -> usize {
        match role {
            TrialRole::Tank => self.max_tanks,
            TrialRole::DD => self.max_dds,
            TrialRole::Healer => self.max_healers
        }
    }
}

impl TryFrom<Message> for TrialData {
    type Error = anyhow::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let trial_embed = value.embeds.first().unwrap().clone();
        let fields = &trial_embed.fields;
        let datetime = fields.get(0).unwrap().value.clone()
            .replace("<t:", "")
            .replace(":f>", "")
            .parse::<i64>().ok();
        let tanks = fields.get(7).unwrap();
        let dds = fields.get(8).unwrap();
        let healers = fields.get(9).unwrap();
        let reserves = fields.get(10).unwrap();
        let absents = fields.get(11).unwrap();

        Ok(TrialData {
            title: trial_embed.title.clone().unwrap(),
            description: trial_embed.description.clone().unwrap(),
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
            duration: fields.get(1).unwrap().value.parse::<DurationString>().unwrap(),
            leader: parse_player(&fields.get(2).unwrap().value).into(),
            guides: empty_to_option(fields.get(3).unwrap().value.clone()),
            addons: empty_to_option(fields.get(4).unwrap().value.clone()),
            tanks: tanks.value.clone().lines().map(|s| parse_player(s)).collect(),
            max_tanks: get_max(&tanks.name).parse::<usize>().unwrap(),
            dds: dds.value.clone().lines().map(|s| parse_player(s)).collect(),
            max_dds: get_max(&dds.name).parse::<usize>().unwrap(),
            healers: healers.value.clone().lines().map(|s| parse_player(s)).collect(),
            max_healers: get_max(&healers.name).parse::<usize>().unwrap(),
            reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
            absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
        })
    }
}