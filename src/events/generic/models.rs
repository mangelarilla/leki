use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateEmbed, Message, UserId};
use crate::events::generic::embeds::event_generic_embed;
use crate::events::models::{EventBasicData, EventEmbed, EventSignups, FromBasicModal, Player, PlayersInRole, remove_from_role};
use crate::events::parse::{parse_basic_from_modal, parse_player, parse_players_in_role};
use crate::events::signup::EventBackupRoles;

#[derive(Debug)]
pub struct EventGenericData {
    title: String,
    description: String,
    pub(crate) datetime: Option<DateTime<Utc>>,
    duration: DurationString,
    leader: UserId,
    signed: PlayersInRole,
    reserves: PlayersInRole,
    absents: PlayersInRole,
}

impl EventGenericData {
    pub fn signup(&mut self, user: UserId) {
        self.remove_signup(user);
        self.signed.push(Player::Basic(user));
    }
}

impl FromBasicModal for EventGenericData {
    fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self {
        let (title, description, duration) = parse_basic_from_modal(components);

        EventGenericData {
            title,
            description,
            duration,
            leader,
            signed: PlayersInRole::default(),
            absents: PlayersInRole::default(),
            reserves: PlayersInRole::default(),
            datetime: None
        }
    }
}

impl EventEmbed for EventGenericData {
    fn get_embed(&self) -> CreateEmbed {
        event_generic_embed(self, false)
    }

    fn get_embed_preview(&self) -> CreateEmbed {
        event_generic_embed(self, true)
    }
}

impl EventBasicData for EventGenericData {
    fn title(&self) -> String {self.title.to_string()}
    fn description(&self) -> String {self.description.clone()}
    fn datetime(&self) -> Option<DateTime<Utc>> {self.datetime.clone()}
    fn duration(&self) -> DurationString {self.duration.clone()}
    fn leader(&self) -> UserId {self.leader.clone()}
}

impl EventBackupRoles for EventGenericData {
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

impl EventSignups for EventGenericData {
    fn signups(&self) -> Vec<Player> {self.signed.clone().into()}
    fn remove_signup(&mut self, user: UserId) {
        remove_from_role(&mut self.signed, user);
        remove_from_role(&mut self.absents, user);
        remove_from_role(&mut self.reserves, user);
    }
}

impl TryFrom<Message> for EventGenericData {
    type Error = anyhow::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let event_embed = value.embeds.first().unwrap();
        let fields = &event_embed.fields;
        let datetime = fields.get(0).unwrap().value.clone()
            .replace("<t:", "")
            .replace(":f>", "")
            .parse::<i64>().ok();
        let signed = fields.get(5).unwrap();
        let reserves = fields.get(6).unwrap();
        let absents = fields.get(7).unwrap();

        Ok(EventGenericData {
            title: event_embed.title.clone().unwrap(),
            description: event_embed.description.clone().unwrap(),
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
            duration: fields.get(1).unwrap().value.parse::<DurationString>().unwrap(),
            leader: parse_player(&fields.get(2).unwrap().value).into(),
            signed: parse_players_in_role(signed),
            reserves: parse_players_in_role(reserves),
            absents: parse_players_in_role(absents),
        })
    }
}