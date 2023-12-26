use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, CreateEmbed, Message, UserId};
use crate::events::models::{EventBasicData, EventComp, EventEmbed, EventRole, EventSignups, FromBasicModal, FromComp, Player, PlayersInRole, remove_from_role};
use crate::events::parse::{empty_to_option, parse_basic_from_modal, parse_player, parse_players_in_role};
use crate::events::signup::{EventBackupRoles, EventSignupRoles};
use crate::events::trials::components::trial_new_comp_components;
use crate::events::trials::embeds::{trial_comp_defaults, trial_embed};
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
    tanks: PlayersInRole,
    dds: PlayersInRole,
    healers: PlayersInRole,
    reserves: PlayersInRole,
    absents: PlayersInRole,
}

impl TrialData {
    pub fn guides(&self) -> Option<String> {self.guides.clone()}
    pub fn addons(&self) -> Option<String> {self.addons.clone()}
}

impl FromBasicModal for TrialData {
    fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self {
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
            tanks: PlayersInRole::new(vec![], Some(2)),
            dds: PlayersInRole::new(vec![], Some(8)),
            healers: PlayersInRole::new(vec![], Some(2)),
            reserves: PlayersInRole::default(),
            absents: PlayersInRole::default()
        }
    }
}

impl FromComp for TrialData {
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
}

impl EventEmbed for TrialData {
    fn get_embed(&self) -> CreateEmbed {
        trial_embed(self, false)
    }

    fn get_embed_preview(&self) -> CreateEmbed {
        trial_embed(self, true)
    }
}

impl EventComp for TrialData {
    fn get_comp_defaults_embed() -> CreateEmbed {
        trial_comp_defaults()
    }

    fn get_comp_new_components() -> Vec<CreateActionRow> {
        trial_new_comp_components()
    }
}

impl EventBasicData for TrialData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> String {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> UserId {self.leader.clone()}
}

impl EventBackupRoles for TrialData {
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

impl EventSignups for TrialData {
    fn signups(&self) -> Vec<Player> {
        [
            self.tanks.clone().as_slice(),
            self.dds.clone().as_slice(),
            self.healers.clone().as_slice()
        ].concat()
    }

    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut self.tanks, user);
        remove_from_role(&mut self.dds, user);
        remove_from_role(&mut self.healers, user);
        remove_from_role(&mut self.reserves, user);
        remove_from_role(&mut self.absents, user);
    }
}

impl EventSignupRoles for TrialData {
    fn is_role_full(&self, role: EventRole) -> bool {
        match role {
            EventRole::Tank => self.tanks.is_role_full(),
            EventRole::DD => self.dds.is_role_full(),
            EventRole::Healer => self.healers.is_role_full(),
            _ => true
        }
    }

    fn signup(&mut self, role: EventRole, player: Player) {
        self.remove_signup(player.clone().into());
        match role {
            EventRole::Tank => self.tanks.push(player),
            EventRole::DD => self.dds.push(player),
            EventRole::Healer => self.healers.push(player),
            _ => ()
        }
    }

    fn role(&self, role: EventRole) -> &PlayersInRole {
        match role {
            EventRole::Tank => &self.tanks,
            EventRole::DD => &self.dds,
            EventRole::Healer => &self.healers,
            _ => unreachable!("No role for Trial")
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
            .replace(":F>", "")
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
            tanks: parse_players_in_role(tanks),
            dds: parse_players_in_role(dds),
            healers: parse_players_in_role(healers),
            reserves: parse_players_in_role(reserves),
            absents: parse_players_in_role(absents),
        })
    }
}