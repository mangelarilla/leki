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
pub struct TrialData {
    title: String,
    description: String,
    datetime: Option<DateTime<Utc>>,
    duration: DurationString,
    leader: UserId,
    tanks: PlayersInRole,
    dds: PlayersInRole,
    healers: PlayersInRole,
    reserves: PlayersInRole,
    absents: PlayersInRole,
}

fn trial_embed(data: &TrialData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_with_role(embed, data, EventSignedRole::Tank);
    let embed = format_with_role(embed, data, EventSignedRole::DD);
    let embed = format_with_role(embed, data, EventSignedRole::Healer);

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
}

impl EventData for TrialData {
    fn prefix() -> &'static str {
        "trial"
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

        TrialData {
            title,
            description,
            datetime: None,
            duration,
            leader,
            tanks: PlayersInRole::new(vec![], Some(2)),
            dds: PlayersInRole::new(vec![], Some(8)),
            healers: PlayersInRole::new(vec![], Some(2)),
            reserves: PlayersInRole::default(),
            absents: PlayersInRole::default()
        }
    }
    fn from_comp_with_preview(components: &Vec<ActionRow>, message: Message) -> Self {
        let mut trial = TrialData::try_from(message).unwrap();

        let tanks = get_input_value(components, 0);
        let dds = get_input_value(components, 1);
        let healers = get_input_value(components, 2);

        trial.tanks = PlayersInRole::new(vec![], tanks.map(|m| m.parse::<usize>().ok()).flatten());
        trial.dds = PlayersInRole::new(vec![], dds.map(|m| m.parse::<usize>().ok()).flatten());
        trial.healers = PlayersInRole::new(vec![], healers.map(|m| m.parse::<usize>().ok()).flatten());

        trial
    }
    fn get_embed(&self) -> CreateEmbed {
        trial_embed(self, false)
    }
    fn get_embed_preview(&self) -> CreateEmbed {
        trial_embed(self, true)
    }
    fn get_comp_defaults_embed() -> CreateEmbed {
        CreateEmbed::new()
            .title("Composicion por defecto")
            .field("Tanks", "2", true)
            .field("DD", "8", true)
            .field("Healers", "2", true)
    }
    fn get_comp_new_components() -> Vec<CreateActionRow> {
        vec![
            short_input("Max Tanks", "trial_max_tanks", "2", false),
            short_input("Max DD", "trial_max_dd", "8", false),
            short_input("Max Healers", "trial_max_healers", "2", false)
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
        vec![EventSignedRole::Tank, EventSignedRole::DD, EventSignedRole::Healer]
    }
    fn is_role_full(&self, role: EventSignedRole) -> bool {
        match role {
            EventSignedRole::Tank => self.tanks.is_role_full(),
            EventSignedRole::DD => self.dds.is_role_full(),
            EventSignedRole::Healer => self.healers.is_role_full(),
            _ => true
        }
    }
    fn signup(&mut self, role: EventSignedRole, player: Player) {
        self.remove_signup(player.clone().into());
        match role {
            EventSignedRole::Tank => self.tanks.push(player),
            EventSignedRole::DD => self.dds.push(player),
            EventSignedRole::Healer => self.healers.push(player),
            _ => ()
        }
    }
    fn role(&self, role: EventSignedRole) -> &PlayersInRole {
        match role {
            EventSignedRole::Tank => &self.tanks,
            EventSignedRole::DD => &self.dds,
            EventSignedRole::Healer => &self.healers,
            _ => unreachable!("No role for Trial")
        }
    }
    fn signups(&self) -> Vec<Player> {
        [
            self.tanks.clone().as_slice(),
            self.dds.clone().as_slice(),
            self.healers.clone().as_slice()
        ].concat()
    }
    fn remove_signup(&mut self, user: UserId) {
        self.tanks.remove_from_role(user);
        self.dds.remove_from_role(user);
        self.healers.remove_from_role(user);
        self.reserves.remove_from_role(user);
        self.absents.remove_from_role(user);
    }
}

impl TryFrom<Message> for TrialData {
    type Error = Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let trial_embed = value.embeds.first().ok_or(Error::ParseEvent)?.clone();
        let fields = &trial_embed.fields;
        let datetime = fields.get(0).ok_or(Error::ParseEvent)?.value.clone()
            .replace("<t:", "")
            .replace(":F>", "")
            .parse::<i64>().ok();
        let tanks = fields.iter().find(|&p| p.name.contains("Tanks")).ok_or(Error::ParseEvent)?;
        let dds = fields.iter().find(|&p| p.name.contains("DD")).ok_or(Error::ParseEvent)?;
        let healers = fields.iter().find(|&p| p.name.contains("Healers")).ok_or(Error::ParseEvent)?;
        let reserves = fields.iter().find(|&p| p.name.contains("Reservas")).ok_or(Error::ParseEvent)?;
        let absents = fields.iter().find(|&p| p.name.contains("Ausencias")).ok_or(Error::ParseEvent)?;
        Ok(TrialData {
            title: trial_embed.title.clone().ok_or(Error::ParseEvent)?,
            description: trial_embed.description.clone().ok_or(Error::ParseEvent)?,
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0)).flatten(),
            duration: fields.get(1).ok_or(Error::ParseEvent)?.value.parse::<DurationString>().map_err(|e| Error::DurationParse(e))?,
            leader: parse_player(&fields.get(2).ok_or(Error::ParseEvent)?.value).ok_or(Error::ParseEvent)?.into(),
            tanks: parse_players_in_role(tanks),
            dds: parse_players_in_role(dds),
            healers: parse_players_in_role(healers),
            reserves: parse_players_in_role(reserves),
            absents: parse_players_in_role(absents),
        })
    }
}