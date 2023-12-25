use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::error::Error;
use crate::events::generic::embeds::event_generic_embed;
use crate::events::generic::models::EventGenericData;
use crate::prelude::*;

const PREFIX: &'static str = "generic_";

pub(super) fn handle_component(interaction: &ComponentInteraction, _ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "event" => Ok(signup_generic(interaction)),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn signup_generic(interaction: &ComponentInteraction) -> CreateInteractionResponse {
    let mut data = EventGenericData::try_from(*interaction.message.clone()).unwrap();
    data.signup(interaction.user.id);
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event_generic_embed(&data, false))
    )
}