use std::marker::PhantomData;
use serenity::all::{CreateInteractionResponse, CreateModal};
use crate::events::EventData;
use crate::messages::BotInteractionMessage;
use crate::prelude::*;

pub(crate) struct UpdateComp<T: EventData + Send> {
    modal_id: String,
    phantom: PhantomData<T>
}

impl<T: EventData + Send> UpdateComp<T> {
    pub(crate) fn new(modal_id: impl Into<String>) -> Self {
        UpdateComp { modal_id: modal_id.into(), phantom: PhantomData }
    }
}

impl<T: EventData + Send> BotInteractionMessage for UpdateComp<T> {
    fn message(&self) -> Result<CreateInteractionResponse> {
        let response = CreateModal::new(&self.modal_id, "Nueva Composicion")
            .components(T::get_comp_new_components());

        Ok(CreateInteractionResponse::Modal(response))
    }
}