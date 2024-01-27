use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, CreateEmbed, Message, UserId};
use crate::error::Error;
use crate::events::embeds::format_with_role;
use crate::events::{EventData, EventSignedRole, Player, PlayersInRole};
use crate::events::parse::{parse_basic_from_modal, parse_player, parse_players_in_role};
use crate::prelude::components::short_input;
use crate::prelude::embeds::{event_embed_backup, event_embed_basic};
use crate::prelude::get_input_value;

#[derive(Debug)]
pub struct PvPData {
    title: String,
    description: String,
    datetime: Option<DateTime<Utc>>,
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

fn pvp_embed(data: &PvPData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_with_role(embed, data, EventSignedRole::Tank);
    let embed = format_with_role(embed, data, EventSignedRole::Brawler);
    let embed = format_with_role(embed, data, EventSignedRole::Healer);
    let embed = format_with_role(embed, data, EventSignedRole::Bomber);
    let embed = format_with_role(embed, data, EventSignedRole::Ganker);

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png")
}

impl EventData for PvPData {
    fn prefix() -> &'static str {
        "pvp"
    }
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> String {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> UserId {self.leader.clone()}
    fn set_datetime(&mut self, dt: DateTime<Utc>) {
        self.datetime = Some(dt);
    }
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
    fn get_embed(&self) -> CreateEmbed {
        pvp_embed(self, false)
    }
    fn get_embed_preview(&self) -> CreateEmbed {
        pvp_embed(self, true)
    }
    fn get_comp_defaults_embed() -> CreateEmbed {
        CreateEmbed::new()
            .title("Composicion por defecto")
            .description("No hay maximos por defecto!")
    }
    fn get_comp_new_components() -> Vec<CreateActionRow> {
        vec![
            short_input("Max Tanks", "pvp_max_tanks", "2 (Por defecto no hay maximo)", false),
            short_input("Max Brawlers", "pvp_max_brawlers", "(Por defecto no hay maximo)", false),
            short_input("Max Healers", "pvp_max_healers", "3 (Por defecto no hay maximo)", false),
            short_input("Max Bombers", "pvp_max_bombers", "(Por defecto no hay maximo)", false),
            short_input("Max Gankers", "pvp_max_gankers", "(Por defecto no hay maximo)", false)
        ]
    }
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
    fn roles() -> Vec<EventSignedRole> {
        vec![EventSignedRole::Tank, EventSignedRole::Healer, EventSignedRole::Brawler, EventSignedRole::Bomber, EventSignedRole::Ganker]
    }
    fn is_role_full(&self, role: EventSignedRole) -> bool {
        match role {
            EventSignedRole::Tank => self.tanks.is_role_full(),
            EventSignedRole::Healer => self.healers.is_role_full(),
            EventSignedRole::Brawler => self.brawlers.is_role_full(),
            EventSignedRole::Bomber => self.bombers.is_role_full(),
            EventSignedRole::Ganker => self.gankers.is_role_full(),
            _ => true
        }
    }
    fn signup(&mut self, role: EventSignedRole, player: Player) {
        self.remove_signup(player.clone().into());
        match role {
            EventSignedRole::Tank => self.tanks.push(player),
            EventSignedRole::Healer => self.healers.push(player),
            EventSignedRole::Brawler => self.brawlers.push(player),
            EventSignedRole::Bomber => self.bombers.push(player),
            EventSignedRole::Ganker => self.gankers.push(player),
            _ => ()
        }
    }
    fn role(&self, role: EventSignedRole) -> &PlayersInRole {
        match role {
            EventSignedRole::Tank => &self.tanks,
            EventSignedRole::Healer => &self.healers,
            EventSignedRole::Brawler => &self.brawlers,
            EventSignedRole::Bomber => &self.bombers,
            EventSignedRole::Ganker => &self.gankers,
            _ => unreachable!("No role for PVP")
        }
    }
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
        self.tanks.remove_from_role(user);
        self.brawlers.remove_from_role(user);
        self.healers.remove_from_role(user);
        self.bombers.remove_from_role(user);
        self.gankers.remove_from_role(user);
        self.gankers.remove_from_role(user);
        self.absents.remove_from_role(user);
    }
}

impl TryFrom<Message> for PvPData {
    type Error = crate::error::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let embed = value.embeds.first().ok_or(Error::ParseEvent)?;
        let fields = &embed.fields;
        let datetime = fields.get(0).unwrap().value.clone()
            .replace("<t:", "")
            .replace(":F>", "")
            .parse::<i64>().ok();
        let tanks = fields.get(5).ok_or(Error::ParseEvent)?;
        let brawlers = fields.get(6).ok_or(Error::ParseEvent)?;
        let healers = fields.get(7).ok_or(Error::ParseEvent)?;
        let bombers = fields.get(8).ok_or(Error::ParseEvent)?;
        let gankers = fields.get(9).ok_or(Error::ParseEvent)?;
        let reserves = fields.get(10).ok_or(Error::ParseEvent)?;
        let absents = fields.get(11).ok_or(Error::ParseEvent)?;

        Ok(PvPData {
            title: embed.title.clone().ok_or(Error::ParseEvent)?,
            description: embed.description.clone().ok_or(Error::ParseEvent)?,
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0)).flatten(),
            duration: fields.get(1).ok_or(Error::ParseEvent)?.value.parse::<DurationString>().map_err(|e| Error::DurationParse(e))?,
            leader: parse_player(&fields.get(2).ok_or(Error::ParseEvent)?.value).ok_or(Error::ParseEvent)?.into(),
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