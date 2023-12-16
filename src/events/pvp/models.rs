use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, Message, UserId};
use crate::events::models::{EventBasicData, EventSignups, Player, remove_from_role};
use crate::events::parse::{get_max, parse_basic_from_modal, parse_player};
use crate::events::pvp::PvPRole;
use crate::events::signup::{EventBackupRoles, EventSignupRoles};

#[derive(Debug)]
pub struct PvPData {
    title: String,
    description: String,
    pub(crate) datetime: Option<DateTime<Utc>>,
    duration: DurationString,
    leader: UserId,
    tanks: Vec<Player>,
    brawlers: Vec<Player>,
    bombers: Vec<Player>,
    healers: Vec<Player>,
    max_tanks: usize,
    max_healers: usize,
    reserves: Vec<Player>,
    absents: Vec<Player>,
}

impl PvPData {
    pub fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self {
        let (title, description, duration) = parse_basic_from_modal(components);
        
        PvPData {
            title,
            description,
            datetime: None,
            duration,
            leader,
            tanks: vec![],
            brawlers: vec![],
            bombers: vec![],
            healers: vec![],
            max_tanks: 2,
            max_healers: 3,
            reserves: vec![],
            absents: vec![],
        }
    }
}

impl EventBasicData for PvPData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> String {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> UserId {self.leader.clone()}
}

impl EventBackupRoles for PvPData {
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

impl EventSignups for PvPData {
    fn signups(&self) -> Vec<Player> {
        [self.tanks.as_slice(), self.brawlers.as_slice(),
            self.bombers.as_slice(), self.healers.as_slice()].concat()
    }

    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut self.tanks, user);
        remove_from_role(&mut self.brawlers, user);
        remove_from_role(&mut self.healers, user);
        remove_from_role(&mut self.bombers, user);
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

    fn signup(&mut self, role: PvPRole, user: UserId) {
        self.remove_signup(user);
        match role {
            PvPRole::Tank => self.tanks.push(Player::Basic(user)),
            PvPRole::Healer => self.healers.push(Player::Basic(user)),
            PvPRole::Brawler => self.brawlers.push(Player::Basic(user)),
            PvPRole::Bomber => self.bombers.push(Player::Basic(user)),
        }
    }

    fn signup_class(&mut self, role: PvPRole, user: UserId, class: String) {
        self.remove_signup(user);
        match role {
            PvPRole::Tank => self.tanks.push(Player::Class(user, class)),
            PvPRole::Healer => self.healers.push(Player::Class(user, class)),
            PvPRole::Brawler => self.brawlers.push(Player::Class(user, class)),
            PvPRole::Bomber => self.bombers.push(Player::Class(user, class)),
        }
    }

    fn role(&self, role: PvPRole) -> &Vec<Player> {
        match role {
            PvPRole::Tank => &self.tanks,
            PvPRole::Healer => &self.healers,
            PvPRole::Brawler => &self.brawlers,
            PvPRole::Bomber => &self.bombers,
        }
    }

    fn max(&self, role: PvPRole) -> usize {
        match role {
            PvPRole::Tank => self.max_tanks,
            PvPRole::Healer => self.max_healers,
            PvPRole::Brawler => 0,
            PvPRole::Bomber => 0
        }
    }
}

impl TryFrom<Message> for PvPData {
    type Error = anyhow::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let embed = value.embeds.first().unwrap();
        let fields = &embed.fields;
        let datetime = fields.get(0).unwrap().value.clone()
            .replace("<t:", "")
            .replace(":f>", "")
            .parse::<i64>().ok();
        let tanks = fields.get(5).unwrap();
        let brawlers = fields.get(6).unwrap();
        let healers = fields.get(7).unwrap();
        let bombers = fields.get(8).unwrap();
        let reserves = fields.get(9).unwrap();
        let absents = fields.get(10).unwrap();

        Ok(PvPData {
            title: embed.title.clone().unwrap(),
            description: embed.description.clone().unwrap(),
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
            duration: fields.get(1).unwrap().value.parse::<DurationString>().unwrap(),
            leader: parse_player(&fields.get(2).unwrap().value).into(),
            tanks: tanks.value.clone().lines().map(|s| parse_player(s)).collect(),
            max_tanks: get_max(&tanks.name).parse::<usize>().unwrap(),
            brawlers: brawlers.value.clone().lines().map(|s| parse_player(s)).collect(),
            bombers: bombers.value.clone().lines().map(|s| parse_player(s)).collect(),
            healers: healers.value.clone().lines().map(|s| parse_player(s)).collect(),
            max_healers: get_max(&healers.name).parse::<usize>().unwrap(),
            reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
            absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
        })
    }
}