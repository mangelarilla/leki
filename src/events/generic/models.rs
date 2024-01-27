use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, CreateEmbed, Message, UserId};
use crate::error::Error;
use crate::events::{EventData, EventSignedRole, Player, PlayersInRole};
use crate::events::embeds::format_with_role;
use crate::events::parse::{parse_basic_from_modal, parse_player, parse_players_in_role};
use crate::prelude::components::short_input;
use crate::prelude::embeds::{event_embed_backup, event_embed_basic};
use crate::prelude::get_input_value;

#[derive(Debug)]
pub struct EventGenericData {
    title: String,
    description: String,
    datetime: Option<DateTime<Utc>>,
    duration: DurationString,
    leader: UserId,
    signed: PlayersInRole,
    reserves: PlayersInRole,
    absents: PlayersInRole
}

impl EventGenericData {
    pub fn signup(&mut self, user: UserId) {
        self.remove_signup(user);
        self.signed.push(Player::Basic(user));
    }
}

fn event_generic_embed(data: &EventGenericData, is_preview: bool) -> CreateEmbed {
    let embed = event_embed_basic(data, is_preview)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false);

    let embed = format_with_role(embed, data, EventSignedRole::Participant);

    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/d/d7/ON-icon-zonestory-assisted.png")
}

impl EventData for EventGenericData {
    fn prefix() -> &'static str {
        "generic"
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

        EventGenericData {
            title,
            description,
            duration,
            leader,
            signed: PlayersInRole::default(),
            reserves: PlayersInRole::default(),
            absents: PlayersInRole::default(),
            datetime: None
        }
    }
    fn from_comp_with_preview(components: &Vec<ActionRow>, message: Message) -> Self {
        let mut event = EventGenericData::try_from(message).unwrap();

        let participants = get_input_value(components, 0);

        event.signed = PlayersInRole::new(vec![], participants.map(|m| m.parse::<usize>().ok()).flatten());

        event
    }
    fn get_embed(&self) -> CreateEmbed {
        event_generic_embed(self, false)
    }
    fn get_embed_preview(&self) -> CreateEmbed {
        event_generic_embed(self, true)
    }
    fn get_comp_defaults_embed() -> CreateEmbed {
        CreateEmbed::new()
            .title("Composicion por defecto")
            .description("No hay maximos por defecto!")
    }
    fn get_comp_new_components() -> Vec<CreateActionRow> {
        vec![
            short_input("Max Participantes", "generic_max_participants", "12 (Por defecto no hay maximo)", false),
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
        vec![EventSignedRole::Participant]
    }
    fn is_role_full(&self, role: EventSignedRole) -> bool {
        match role {
            EventSignedRole::Participant => self.signed.is_role_full(),
            _ => true
        }
    }
    fn signup(&mut self, role: EventSignedRole, player: Player) {
        self.remove_signup(player.clone().into());
        match role {
            EventSignedRole::Participant => self.signed.push(player),
            _ => ()
        }
    }
    fn role(&self, role: EventSignedRole) -> &PlayersInRole {
        match role {
            EventSignedRole::Participant => &self.signed,
            _ => unreachable!("No role for Generic")
        }
    }
    fn signups(&self) -> Vec<Player> {
        self.signed.clone().into()
    }
    fn remove_signup(&mut self, user: UserId) {
        self.signed.remove_from_role(user);
    }
}

impl TryFrom<Message> for EventGenericData {
    type Error = Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let event_embed = value.embeds.first().ok_or(Error::ParseEvent)?;
        let fields = &event_embed.fields;
        let datetime = fields.get(0).ok_or(Error::ParseEvent)?.value.clone()
            .replace("<t:", "")
            .replace(":F>", "")
            .parse::<i64>().ok();
        let signed = fields.get(5).ok_or(Error::ParseEvent)?;
        let reserves = fields.get(6).ok_or(Error::ParseEvent)?;
        let absents = fields.get(7).ok_or(Error::ParseEvent)?;

        Ok(EventGenericData {
            title: event_embed.title.clone().ok_or(Error::ParseEvent)?,
            description: event_embed.description.clone().ok_or(Error::ParseEvent)?,
            datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0)).flatten(),
            duration: fields.get(1).ok_or(Error::ParseEvent)?.value.parse::<DurationString>().map_err(|e| Error::DurationParse(e))?,
            leader: parse_player(&fields.get(2).ok_or(Error::ParseEvent)?.value).ok_or(Error::ParseEvent)?.into(),
            signed: parse_players_in_role(signed),
            reserves: parse_players_in_role(reserves),
            absents: parse_players_in_role(absents),
        })
    }
}