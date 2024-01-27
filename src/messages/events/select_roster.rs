use std::marker::PhantomData;
use serenity::all::{ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, Message};
use crate::events::{EventData, EventSignedRole, Player};
use crate::messages::{BotInteractionFromComponentMessage, BotInteractionMessage};
use crate::prelude::*;
use crate::prelude::components::get_roster_select;

const NEXT_PAGE: &'static str = "role_next";
const PREV_PAGE: &'static str = "role_prev";

pub(crate) struct SelectRoster<T: EventData + Send> {
    label: String,
    roles: Vec<EventSignedRole>,
    confirm_id: String,
    is_first_page: bool,
    selected_role: Option<EventSignedRole>,
    is_reserve: bool,
    phantom: PhantomData<T>
}

impl<T: EventData + Send> SelectRoster<T> {
    pub(crate) fn new(confirm_id: impl Into<String>, label: impl Into<String>, roles: Vec<EventSignedRole>) -> Self {
        Self::new_full(confirm_id, label, roles, true, None, false)
    }
    fn new_with_role(confirm_id: impl Into<String>, label: impl Into<String>, roles: Vec<EventSignedRole>, selected_role: EventSignedRole) -> Self {
        Self::new_full(confirm_id, label, roles, true, Some(selected_role), false)
    }
    fn new_with_reserve(confirm_id: impl Into<String>, label: impl Into<String>, roles: Vec<EventSignedRole>) -> Self {
        Self::new_full(confirm_id, label, roles, true, None, true)
    }
    fn new_full(confirm_id: impl Into<String>, label: impl Into<String>, roles: Vec<EventSignedRole>, is_first_page: bool, selected_role: Option<EventSignedRole>, is_reserve: bool) -> Self {
        SelectRoster {
            label: label.into(),
            confirm_id: confirm_id.into(),
            roles,
            is_first_page,
            phantom: PhantomData,
            selected_role,
            is_reserve
        }
    }

    pub(crate) fn registries(&self) -> Vec<(String, SelectRoster<T>)> {
        let mut entries: Vec<(String, SelectRoster<T>)> = self.roles.iter()
            .map(|r| {
                let id = T::prefix_id(format!("{}_{}", &self.label, r.to_id()));
                let handler = Self::new_with_role(self.confirm_id.to_string(), self.label.to_string(), self.roles.to_vec(), *r);
                (id, handler)
            }).collect();

        entries.push((T::prefix_id(format!("{}_reserve", &self.label)), Self::new_with_reserve(self.confirm_id.to_string(), self.label.to_string(), self.roles.to_vec())));

        if entries.len() > 4 {
            entries.push((self.prev_id(), Self::new(self.confirm_id.to_string(), self.label.to_string(), self.roles.to_vec())));
            entries.push((self.next_id(), Self::new_full(self.confirm_id.to_string(), self.label.to_string(), self.roles.to_vec(), false, None, false)));
        }

        entries
    }

    fn prev_id(&self) -> String {
        T::prefix_id(format!("{}_{}", &self.label, PREV_PAGE))
    }

    fn next_id(&self) -> String {
        T::prefix_id(format!("{}_{}", &self.label, NEXT_PAGE))
    }

    fn get_component_roles(&self) -> Vec<CreateActionRow> {
        let mut roles: Vec<CreateActionRow> = self.roles.iter()
            .map(|r| {
                let id = T::prefix_id(format!("{}_{}", &self.label, r.to_id()));
                get_roster_select(&id, &r.to_string(), 12)
            }).collect();

        roles.push(get_roster_select(&T::prefix_id(format!("{}_reserve", &self.label)), "Reservas", 12));

        if self.is_first_page {
            roles.into_iter().take(4).collect()
        } else {
            roles.into_iter().skip(4).collect()
        }
    }

    fn get_button_row(&self) -> CreateActionRow {
        let confirm = CreateButton::new(&self.confirm_id)
            .label("Continuar")
            .style(ButtonStyle::Primary);

        let buttons = if self.is_first_page && self.roles.len() > 4 {
            vec![
                CreateButton::new(self.next_id())
                    .label("Mas roles >>")
                    .style(ButtonStyle::Secondary), confirm
            ]
        } else if self.roles.len() > 4 {
            vec![
                CreateButton::new(self.prev_id())
                    .label("<< Mas roles")
                    .style(ButtonStyle::Secondary), confirm
            ]
        } else {
            vec![confirm]
        };

        CreateActionRow::Buttons(buttons)
    }
}

impl<T: EventData + Send> BotInteractionMessage for SelectRoster<T> {
    fn message(&self) -> Result<CreateInteractionResponse> {
        let mut roles = self.get_component_roles();
        roles.push(self.get_button_row());

        let response = CreateInteractionResponseMessage::new()
            .components(roles);

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}

impl<T: EventData + Send> BotInteractionFromComponentMessage for SelectRoster<T> where Error: From<<T as TryFrom<Message>>::Error> {
    fn message(&self, interaction: &ComponentInteraction) -> Result<CreateInteractionResponse> {
        let selected_users = get_selected_users(interaction);
        let mut event = T::try_from(*interaction.message.clone())?;

        if let Some(role) = self.selected_role {
            for user in selected_users {
                event.signup(role, Player::Basic(user));
            }
        } else if self.is_reserve {
            for user in selected_users {
                event.add_reserve(Player::Basic(user));
            }
        };

        let mut roles = self.get_component_roles();
        roles.push(self.get_button_row());

        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(event.get_embed_preview())
                .components(roles)
        ))
    }
}