use serenity::all::{CreateInteractionResponse, CreateModal};
use crate::messages::BotInteractionMessage;
use crate::prelude::*;

pub(crate) struct EventInfo {
    modal_id: String
}

impl EventInfo {
    pub(crate) fn new(modal_id: impl Into<String>) -> Self {
        EventInfo { modal_id: modal_id.into() }
    }
}

impl BotInteractionMessage for EventInfo {
    fn message(&self) -> Result<CreateInteractionResponse> {
        let modal = CreateModal::new(&self.modal_id, "Informacion del Evento")
            .components(vec![
                components::short_input("Titulo", "event_title", "Trial nivel avanzado - vRG", true),
                components::short_input("Duracion", "event_duration", "2h", true),
                components::long_input("Descripción", "event_description", "Se empezara a montar 10 minutos antes\nbla bla bla", true),
            ]);

        Ok(CreateInteractionResponse::Modal(modal))
    }
}