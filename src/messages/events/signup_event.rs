use std::marker::PhantomData;
use serenity::all::{ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::events::components::{select_flex_roles, select_player_class};
use crate::events::{EventData, EventRole};
use crate::messages::{BotInteractionFromComponentMessage};
use crate::messages::events::{SignupEventClass};
use crate::prelude::*;

pub struct SignupEvent<T: EventData + Sync + Send> {
    role: EventRole,
    phantom: PhantomData<T>
}

impl<T: EventData + Sync + Send> SignupEvent<T> {
    pub fn new(role: EventRole) -> Self {
        SignupEvent { role, phantom: PhantomData }
    }

    pub fn flex_registry(&self) -> (String, SignupEventClass<T>) {
        (self.flex_id(), SignupEventClass::<T>::new(self.role))
    }

    pub fn class_registry(&self) -> (String, SignupEventClass<T>) {
        (self.class_id(), SignupEventClass::<T>::new(self.role))
    }

    fn flex_id(&self) -> String {
        T::prefix_id(format!("{}_flex", self.role.to_id()))
    }

    fn class_id(&self) -> String {
        T::prefix_id(format!("{}_class", self.role.to_id()))
    }
}

impl<T: EventData + Sync + Send> BotInteractionFromComponentMessage for SignupEvent<T>
    where Error: From<<T as TryFrom<serenity::all::Message>>::Error> {
    fn message(&self, interaction: &ComponentInteraction) -> Result<CreateInteractionResponse> {
        if self.role == EventRole::Absent {
            let mut data = T::try_from(*interaction.message.clone())?;
            data.add_absent(interaction.user.id);
            Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new().embed(data.get_embed())))
        } else {
            let class_selector = select_player_class(self.class_id());
            let flex_selector = select_flex_roles(self.flex_id(), &T::roles());

            Ok(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .components(vec![flex_selector, class_selector])
            ))
        }
    }
}