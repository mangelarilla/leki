use std::marker::PhantomData;
use serenity::all::{ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, Message, ModalInteraction};
use crate::events::EventData;
use crate::prelude::*;
use crate::messages::{BotInteractionFromComponentMessage, BotInteractionModalMessage};

pub(crate) struct EventScope<T: EventData + Send> {
    phantom: PhantomData<T>,
    id_public: String,
    id_semi_public: String,
    id_private: String
}

impl<T: EventData + Send> EventScope<T> {
    pub(crate) fn new(id_public: impl Into<String>, id_semi_public: impl Into<String>, id_private: impl Into<String>) -> Self {
        EventScope::<T> {
            phantom: PhantomData,
            id_public: id_public.into(),
            id_private: id_private.into(),
            id_semi_public: id_semi_public.into()
        }
    }

    fn message(&self, event: T) -> Result<CreateInteractionResponse> {
        let response = CreateInteractionResponseMessage::new()
            .embed(event.get_embed_preview())
            .components(vec![
                CreateActionRow::Buttons(vec![
                    CreateButton::new(&self.id_public)
                        .label("Abierto")
                        .style(ButtonStyle::Success),
                    CreateButton::new(&self.id_semi_public)
                        .label("Semi-abierto")
                        .style(ButtonStyle::Secondary),
                    CreateButton::new(&self.id_private)
                        .label("Cerrado")
                        .style(ButtonStyle::Danger)
                ])
            ]);

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}


impl<T: EventData + Send> BotInteractionFromComponentMessage for EventScope<T> where Error: From<<T as TryFrom<Message>>::Error> {
    fn message(&self, interaction: &ComponentInteraction) -> Result<CreateInteractionResponse> {
        let event = T::try_from(*interaction.message.clone())?;
        self.message(event)
    }
}

impl<T: EventData + Send> BotInteractionModalMessage for EventScope<T> {
    fn message(&self, interaction: &ModalInteraction) -> Result<CreateInteractionResponse> {
        let event = T::from_comp_with_preview(&interaction.data.components, *interaction.message.clone().unwrap());
        self.message(event)
    }
}