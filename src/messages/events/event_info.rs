use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateModal};
use crate::events::EventKind;
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::BotInteractionMessage;
use crate::messages::events::EventComposition;
use crate::prelude::*;

pub(crate) struct EventInfo {
    modal_id: String
}

impl EventInfo {
    pub(crate) fn new(kind: EventKind, pipeline: &mut InteractionPipeline) -> Self {
        let info = EventInfo { modal_id: format!("{kind}_event_info") };
        let comp =  EventComposition::new(kind, pipeline);
        pipeline.add(&info.modal_id, comp);
        info
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for EventInfo {
    async fn component(&self, _interaction: &ComponentInteraction, _ctx: &Context, _store: &Store) -> Result<CreateInteractionResponse> {
        let modal = CreateModal::new(&self.modal_id, "Informacion del Evento")
            .components(vec![
                components::short_input("Titulo", "event_title", "Trial nivel avanzado - vRG", true),
                components::short_input("Duracion", "event_duration", "2h", true),
                components::long_input("Descripción", "event_description", "Se empezara a montar 10 minutos antes\nbla bla bla", true),
            ]);

        Ok(CreateInteractionResponse::Modal(modal))
    }
}