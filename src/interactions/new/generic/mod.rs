use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, ModalInteraction};
use crate::events::components::{select_event_channel};
use crate::events::generic::components::event_generic_basic_info;
use crate::events::generic::embeds::event_generic_embed;
use crate::events::generic::models::EventGenericData;
use crate::prelude::*;

const PREFIX: &'static str = "generic_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "event" => Ok(request_basic_generic_data()),
        "event_day" => Ok(super::request_event_times(&prefixed("times"), ctx, interaction).await?),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

pub(super) async fn handle_modal(interaction: &ModalInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "basic_info" => Ok(request_day_channel_and_create_preview(interaction)),
        "times" => Ok(super::create_event(interaction, ctx, false).await?),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}

fn request_day_channel_and_create_preview(interaction: &ModalInteraction) -> CreateInteractionResponse {
    let event = EventGenericData::from_basic_modal(&interaction.data.components, interaction.user.id);
    let response = CreateInteractionResponseMessage::new()
        .add_embed(event_generic_embed(&event, true))
        .components(select_event_channel(&prefixed("event_day")));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_basic_generic_data() -> CreateInteractionResponse {
    let response = CreateModal::new(&prefixed("basic_info"), "Informacion del Evento")
        .components(event_generic_basic_info());

    CreateInteractionResponse::Modal(response)
}