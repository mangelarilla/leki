use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, CreateEmbed, Message, UserId};
use crate::events::models::{EventBasicData, EventComp, EventEmbed, EventSignups, FromBasicModal, FromComp, Player, PlayersInRole, remove_from_role};
use crate::events::parse::{parse_basic_from_modal, parse_player, parse_players_in_role};
use crate::events::pvp::components::pvp_new_comp_components;
use crate::events::pvp::embeds::{pvp_comp_defaults, pvp_embed};
use crate::events::pvp::PvPRole;
use crate::events::signup::{EventBackupRoles, EventSignupRoles};
use crate::prelude::get_input_value;

#[derive(Debug)]
pub struct PvPData {
    title: String,
    description: String,
    pub(crate) datetime: Option<DateTime<Utc>>,
    duration: DurationString,
    leader: UserId,
    tanks: PlayersInRole,
    brawlers: PlayersInRole,
    bombers: PlayersInRole,
    gankers: PlayersInRole,
    healers: PlayersInRole,
    reserves: PlayersInRole,
    absents: PlayersInRole,
}

impl FromBasicModal for PvPData {
    fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self {
        let (title, description, duration) = parse_basic_from_modal(components);

        PvPData {
            title,
            description,
            datetime: None,
            duration,
            leader,
            tanks: PlayersInRole::default(),
            brawlers: PlayersInRole::default(),
            bombers: PlayersInRole::default(),
            healers: PlayersInRole::default(),
            reserves: PlayersInRole::default(),
            absents: PlayersInRole::default(),
            gankers: PlayersInRole::default(),
        }
    }
}

impl FromComp for PvPData {
    fn from_comp_with_preview(components: &Vec<ActionRow>, message: Message) -> Self {
        let mut pvp = PvPData::try_from(message).unwrap();

        let tanks = get_input_value(components, 0);
        let brawlers = get_input_value(components, 1);
        let healers = get_input_value(components, 2);
        let bombers = get_input_value(components, 2);
        let gankers = get_input_value(components, 2);

        pvp.tanks = PlayersInRole::new(vec![], tanks.map(|m| m.parse::<usize>().ok()).flatten());
        pvp.brawlers = PlayersInRole::new(vec![], brawlers.map(|m| m.parse::<usize>().ok()).flatten());
        pvp.healers = PlayersInRole::new(vec![], healers.map(|m| m.parse::<usize>().ok()).flatten());
        pvp.bombers = PlayersInRole::new(vec![], bombers.map(|m| m.parse::<usize>().ok()).flatten());
        pvp.gankers = PlayersInRole::new(vec![], gankers.map(|m| m.parse::<usize>().ok()).flatten());

        pvp
    }
}

impl EventEmbed for PvPData {
    fn get_embed(&self) -> CreateEmbed {
        pvp_embed(self, false)
    }

    fn get_embed_preview(&self) -> CreateEmbed {
        pvp_embed(self, true)
    }
}

impl EventComp for PvPData {
    fn get_comp_defaults_embed() -> CreateEmbed {
        pvp_comp_defaults()
    }

    fn get_comp_new_components() -> Vec<CreateActionRow> {
        pvp_new_comp_components()
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
    fn reserves(&self) -> Vec<Player> {self.reserves.clone().into()}
    fn absents(&self) -> Vec<Player> {self.absents.clone().into()}
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
        [
            self.tanks.clone().as_slice(),
            self.brawlers.clone().as_slice(),
            self.bombers.clone().as_slice(),
            self.healers.clone().as_slice(),
            self.gankers.clone().as_slice()
        ].concat()
    }

    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut self.tanks, user);
        remove_from_role(&mut self.brawlers, user);
        remove_from_role(&mut self.healers, user);
        remove_from_role(&mut self.bombers, user);
        remove_from_role(&mut self.gankers, user);
        remove_from_role(&mut self.reserves, user);
        remove_from_role(&mut self.absents, user);
    }
}

impl EventSignupRoles<PvPRole> for PvPData {
    fn is_role_full(&self, role: PvPRole) -> bool {
        match role {
            PvPRole::Tank => self.tanks.is_role_full(),
            PvPRole::Healer => self.healers.is_role_full(),
            PvPRole::Brawler => self.brawlers.is_role_full(),
            PvPRole::Bomber => self.bombers.is_role_full(),
            PvPRole::Ganker => self.gankers.is_role_full()
        }
    }

    fn signup(&mut self, role: PvPRole, user: UserId) {
        self.remove_signup(user);
        match role {
            PvPRole::Tank => self.tanks.push(Player::Basic(user)),
            PvPRole::Healer => self.healers.push(Player::Basic(user)),
            PvPRole::Brawler => self.brawlers.push(Player::Basic(user)),
            PvPRole::Bomber => self.bombers.push(Player::Basic(user)),
            PvPRole::Ganker => self.gankers.push(Player::Basic(user))
        }
    }

    fn signup_class(&mut self, role: PvPRole, user: UserId, class: String) {
        self.remove_signup(user);
        match role {
            PvPRole::Tank => self.tanks.push(Player::Class(user, class)),
            PvPRole::Healer => self.healers.push(Player::Class(user, class)),
            PvPRole::Brawler => self.brawlers.push(Player::Class(user, class)),
            PvPRole::Bomber => self.bombers.push(Player::Class(user, class)),
            PvPRole::Ganker => self.gankers.push(Player::Class(user, class)),
        }
    }

    fn role(&self, role: PvPRole) -> &PlayersInRole {
        match role {
            PvPRole::Tank => &self.tanks,
            PvPRole::Healer => &self.healers,
            PvPRole::Brawler => &self.brawlers,
            PvPRole::Bomber => &self.bombers,
            PvPRole::Ganker => &self.gankers
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
        let gankers = fields.get(9).unwrap();
        let reserves = fields.get(10).unwrap();
        let absents = fields.get(11).unwrap();

        Ok(PvPData {
            title: embed.title.clone().unwrap(),
            description: embed.description.clone().unwrap(),
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
            duration: fields.get(1).unwrap().value.parse::<DurationString>().unwrap(),
            leader: parse_player(&fields.get(2).unwrap().value).into(),
            tanks: parse_players_in_role(tanks),
            brawlers: parse_players_in_role(brawlers),
            bombers: parse_players_in_role(bombers),
            gankers: parse_players_in_role(gankers),
            healers: parse_players_in_role(healers),
            reserves: parse_players_in_role(reserves),
            absents: parse_players_in_role(absents),
        })
    }
}