use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, CreateEmbed, Message, UserId};
use crate::events::models::{EventBasicData, EventComp, EventEmbed, EventRole, EventSignups, FromBasicModal, FromComp, Player, PlayersInRole, remove_from_role};
use crate::events::parse::{parse_basic_from_modal, parse_player, parse_players_in_role};
use crate::events::pvp::components::pvp_new_comp_components;
use crate::events::pvp::embeds::{pvp_comp_defaults, pvp_embed};
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
        let bombers = get_input_value(components, 3);
        let gankers = get_input_value(components, 4);

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
    fn add_reserve(&mut self, player: Player) {
        self.remove_signup(player.clone().into());
        self.reserves.push(player)
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

impl EventSignupRoles for PvPData {
    fn is_role_full(&self, role: EventRole) -> bool {
        match role {
            EventRole::Tank => self.tanks.is_role_full(),
            EventRole::Healer => self.healers.is_role_full(),
            EventRole::Brawler => self.brawlers.is_role_full(),
            EventRole::Bomber => self.bombers.is_role_full(),
            EventRole::Ganker => self.gankers.is_role_full(),
            _ => true
        }
    }

    fn signup(&mut self, role: EventRole, player: Player) {
        self.remove_signup(player.clone().into());
        match role {
            EventRole::Tank => self.tanks.push(player),
            EventRole::Healer => self.healers.push(player),
            EventRole::Brawler => self.brawlers.push(player),
            EventRole::Bomber => self.bombers.push(player),
            EventRole::Ganker => self.gankers.push(player),
            _ => ()
        }
    }

    fn role(&self, role: EventRole) -> &PlayersInRole {
        match role {
            EventRole::Tank => &self.tanks,
            EventRole::Healer => &self.healers,
            EventRole::Brawler => &self.brawlers,
            EventRole::Bomber => &self.bombers,
            EventRole::Ganker => &self.gankers,
            _ => unreachable!("No role for PVP")
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
            .replace(":F>", "")
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