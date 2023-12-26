use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse};
use crate::events::models::EventRole;
use crate::events::trials::models::TrialData;
use crate::interactions::signup::signup_class_flex;
use crate::prelude::*;

const PREFIX: &'static str = "trial_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let trial_roles = &[EventRole::DD, EventRole::Healer, EventRole::Tank];

    let response = match event_id.as_str() {
        "tank" => signup_class_flex(prefixed("tank"), trial_roles),
        "dd" => signup_class_flex(prefixed("dd"), trial_roles),
        "healer" => signup_class_flex(prefixed("healer"), trial_roles),
        "reserve" => signup_class_flex(prefixed("reserve"), trial_roles),
        "healer_flex" | "dd_flex" | "tank_flex" | "reserve_flex" => super::update_flex_preview(interaction),
        "reserve_class" => super::signup_confirm::<TrialData>(interaction, ctx, None).await,
        "healer_class" => super::signup_confirm::<TrialData>(interaction, ctx, Some(EventRole::Healer)).await,
        "dd_class" => super::signup_confirm::<TrialData>(interaction, ctx, Some(EventRole::DD)).await,
        "tank_class" => super::signup_confirm::<TrialData>(interaction, ctx, Some(EventRole::Tank)).await,
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}