use std::fmt::Display;
use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ActionRow, CreateActionRow, CreateButton, CreateEmbed, Message, UserId};
use super::{EventRole, EventSignedRole, Player, PlayersInRole};

pub trait EventData: TryFrom<Message> {
    fn prefix() -> &'static str;
    fn prefix_id(id: impl Into<String>+Display) -> String {
        format!("{}_{id}", Self::prefix())
    }
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn datetime(&self) -> Option<DateTime<Utc>>;
    fn duration(&self) -> DurationString;
    fn leader(&self) -> UserId;
    fn set_datetime(&mut self, dt: DateTime<Utc>);
    fn from_basic_modal(components: &Vec<ActionRow>, leader: UserId) -> Self;
    fn from_comp_with_preview(components: &Vec<ActionRow>, message: Message) -> Self;
    fn get_embed(&self) -> CreateEmbed;
    fn get_embed_preview(&self) -> CreateEmbed;
    fn get_comp_defaults_embed() -> CreateEmbed;
    fn get_comp_new_components() -> Vec<CreateActionRow>;
    fn backup_buttons() -> Vec<CreateButton> {
        let reserve = EventRole::Reserve;
        let absent = EventRole::Absent;
        vec![
            reserve.to_button(&Self::prefix_id(reserve.to_id())),
            absent.to_button(&Self::prefix_id(absent.to_id())),
        ]
    }
    fn reserves(&self) -> Vec<Player>;
    fn absents(&self) -> Vec<Player>;
    fn add_absent(&mut self, user: UserId);
    fn add_reserve(&mut self, player: Player);
    fn roles() -> Vec<EventSignedRole>;
    fn role_buttons() -> Vec<CreateButton> {
        Self::roles()
            .into_iter()
            .map(|r| r.to_button(&Self::prefix_id(r.to_id())))
            .collect()
    }
    fn is_role_full(&self, role: EventSignedRole) -> bool;
    fn signup(&mut self, role: EventSignedRole, player: Player);
    fn role(&self, role: EventSignedRole) -> &PlayersInRole;
    fn signups(&self) -> Vec<Player>;
    fn remove_signup(&mut self, user: UserId);
}