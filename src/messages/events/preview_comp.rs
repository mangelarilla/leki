use std::marker::PhantomData;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, ModalInteraction};
use crate::events::components::event_comp_defaults_components;
use crate::events::EventData;
use crate::messages::{BotInteractionModalMessage};
use crate::prelude::*;

pub(crate) struct PreviewComp<T: EventData + Send> {
    confirm: String,
    modify: String,
    phantom: PhantomData<T>
}

impl<T: EventData + Send> PreviewComp<T> {
    pub(crate) fn new(confirm_id: impl Into<String>, modify_id: impl Into<String>) -> Self {
        PreviewComp { confirm: confirm_id.into(), modify: modify_id.into(), phantom: PhantomData}
    }
}

impl<T: EventData + Send> BotInteractionModalMessage for PreviewComp<T> {
    fn message(&self, interaction: &ModalInteraction) -> Result<CreateInteractionResponse> {
        let event = T::from_basic_modal(&interaction.data.components, interaction.user.id);
        let response = CreateInteractionResponseMessage::new()
            .add_embed(event.get_embed_preview())
            .add_embed(T::get_comp_defaults_embed())
            .components(event_comp_defaults_components(&self.confirm, &self.modify));

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}