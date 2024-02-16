use anyhow::Context as AnyhowContext;
use duration_string::DurationString;
use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateModal, Message, ModalInteraction};
use crate::events::{Event, EventKind};
use crate::prelude::*;

pub(super) async fn request_info_modal(message: &Message, interaction: &ComponentInteraction, ctx: &Context, kind: EventKind) -> Result<(ModalInteraction, Event)> {
    interaction.create_response(&ctx.http, create_event_info_modal()).await?;
    if let Some(modal) = message.await_modal_interaction(&ctx.shard).await {
        let (title, duration, description) = parse_info_modal(&modal)?;
        let event = Event::new(title, duration, description, interaction.user.id, kind);
        Ok((modal, event))
    } else {
        Err(Error::Timeout)
    }
}

fn create_event_info_modal() -> CreateInteractionResponse {
    CreateInteractionResponse::Modal(
        CreateModal::new("create_event_info", "Informacion del Evento")
            .components(vec![
                components::short_input("Titulo", "event_title", "Trial nivel avanzado - vRG", true),
                components::short_input("Duracion", "event_duration", "2h", true),
                components::long_input("Descripción", "event_description", "Se empezara a montar 10 minutos antes\nbla bla bla", true),
            ])
    )
}

fn parse_info_modal(modal: &ModalInteraction) -> Result<(String, DurationString, String)> {
    let title = get_input_value(&modal.data.components, 0).context("title")?;
    let duration = get_input_value(&modal.data.components, 1)
        .context("duration")?
        .parse::<DurationString>().map_err(|e| Error::DurationParse(e))?;
    let description = get_input_value(&modal.data.components, 2)
        .context("description")?;

    Ok((title, duration, description))
}