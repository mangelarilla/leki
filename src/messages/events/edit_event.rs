use serenity::all::{ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, MessageId};
use strum::IntoEnumIterator;
use crate::events::{Event, EventRole};
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::{BotInteractionMessage};
use crate::messages::events::edit_event_role::EditEventRole;
use crate::prelude::*;

pub struct EditEvent;

impl EditEvent {
    pub fn new(pipeline: &mut InteractionPipeline) -> Self {
        for role in EventRole::iter() {
            let event_role = EditEventRole::new(role);
            pipeline.add(format!("edit_{}", role.to_id()), event_role.clone());
            pipeline.add(event_role.select_id(), event_role);
        }

        EditEvent
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for EditEvent {
    async fn command(&self, interaction: &CommandInteraction, _ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let message = interaction.data.resolved.messages.values().next().unwrap();

        if let Ok(event) = store.get_event(message.id).await {
            Ok(CreateInteractionResponse::Message(edit_event(event, message.id)))
        } else {
            Ok(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .content("Eso no es un evento atontao!")
                .ephemeral(true)
            ))
        }
    }
}

pub(super) fn edit_event(event: Event, msg_id: MessageId) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .embed(event.embed())
        .components(vec![
            CreateActionRow::Buttons(event.roles.iter()
                .filter_map(|pr| if pr.role.is_backup_role() {None} else { Some(edit_button(&pr.role, msg_id))}).collect()),
            CreateActionRow::Buttons(event.roles.iter()
                .filter_map(|pr| if !pr.role.is_backup_role() {None} else { Some(edit_button(&pr.role, msg_id)
                    .style(ButtonStyle::Secondary))}).collect()),
        ])
}

fn edit_button(role: &EventRole, msg_id: MessageId) -> CreateButton {
    CreateButton::new(format!("edit_{}__{}", role.to_id(), msg_id.to_string()))
        .label(format!("Mover a {role}"))
        .emoji(role.emoji())
        .style(ButtonStyle::Success)
}