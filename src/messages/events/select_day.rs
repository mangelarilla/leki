use std::marker::PhantomData;
use serenity::all::{ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage, Message};
use crate::error::Error;
use crate::events::components::select_event_channel;
use crate::events::EventData;
use crate::messages::BotInteractionFromComponentMessage;

pub(crate) struct SelectDay<T: EventData + Send> {
    day_id: String,
    phantom: PhantomData<T>,
    is_private: bool
}

impl<T: EventData + Send> SelectDay<T> {
    pub(crate) fn new(day_id: impl Into<String>, is_private: bool) -> Self {
        SelectDay { day_id: day_id.into(), phantom: PhantomData, is_private }
    }
}

impl<T: EventData + Send> BotInteractionFromComponentMessage for SelectDay<T> where Error: From<<T as TryFrom<Message>>::Error> {
    fn message(&self, interaction: &ComponentInteraction) -> crate::prelude::Result<CreateInteractionResponse> {
        let event = T::try_from(*interaction.message.clone())?;
        let response = CreateInteractionResponseMessage::new()
            .embed(if self.is_private {
                event.get_embed_preview()
                    .title(format!("[Roster Cerrado] {}", event.title()))
            } else { event.get_embed_preview() })
            .components(select_event_channel(&self.day_id));

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}