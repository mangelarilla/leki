use serenity::all::{ButtonStyle, CommandInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::events::{EventData, EventRole};
use crate::events::generic::models::EventGenericData;
use crate::events::pvp::models::PvPData;
use crate::events::trials::models::TrialData;
use crate::messages::{BotInteractionFromCommandMessage};
use crate::prelude::*;

pub struct EditEvent;

impl EditEvent {
    pub fn new() -> Self {EditEvent}
}

impl BotInteractionFromCommandMessage for EditEvent {
    fn message(&self, interaction: &CommandInteraction) -> Result<CreateInteractionResponse> {
        let message = interaction.data.resolved.messages.values().next().unwrap();
        if message.author.id != 1148032756899643412 {
            return Ok(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .content("Eso no es un evento atontao!")
                .ephemeral(true)
            ));
        }

        if let Ok(trial) = TrialData::try_from(message.clone()) {
            Ok(CreateInteractionResponse::Message(edit_event(trial)))
        }
        else if let Ok(pvp) = PvPData::try_from(message.clone()) {
            Ok(CreateInteractionResponse::Message(edit_event(pvp)))
        }
        else if let Ok(generic) = EventGenericData::try_from(message.clone()) {
            Ok(CreateInteractionResponse::Message(edit_event(generic)))
        } else {
            Err(Error::ParseEvent)
        }
    }
}

pub(super) fn edit_event<T: EventData>(event: T) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .embed(event.get_embed())
        .components(vec![
            CreateActionRow::Buttons(T::roles().into_iter()
                .map(|r| edit_button::<T>("edit", EventRole::Signed(r), ButtonStyle::Success)).collect()),
            CreateActionRow::Buttons(vec![edit_button::<T>("edit", EventRole::Reserve, ButtonStyle::Secondary)])
        ])
}

pub(super) fn edit_button<T: EventData>(prefix: &str, role: EventRole, style: ButtonStyle) -> CreateButton {
    CreateButton::new(T::prefix_id(format!("{prefix}_{}", role.to_id())))
        .label(format!("Mover a {role}"))
        .emoji(role.emoji())
        .style(style)
}
