use log::info;
use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse};
use crate::error::Error;
use crate::events::models::{EventRole};
use crate::events::pvp::models::PvPData;
use crate::interactions::signup::signup_class_flex;
use crate::prelude::*;

const PREFIX: &'static str = "pvp_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");
    info!("PvP signup interaction: {event_id}");
    let pvp_roles = &[EventRole::Ganker, EventRole::Bomber, EventRole::Healer, EventRole::Brawler, EventRole::Tank];

    let response = match event_id.as_str() {
        "tank" => signup_class_flex(prefixed("tank"), pvp_roles),
        "brawler" => signup_class_flex(prefixed("brawler"), pvp_roles),
        "healer" => signup_class_flex(prefixed("healer"), pvp_roles),
        "bomber" => signup_class_flex(prefixed("bomber"), pvp_roles),
        "ganker" => signup_class_flex(prefixed("ganker"), pvp_roles),
        "reserve" => signup_class_flex(prefixed("reserve"), pvp_roles),
        "healer_flex" | "brawler_flex" | "tank_flex" | "bomber_flex" | "ganker_flex" | "reserve_flex" => super::update_flex_preview(interaction),
        "reserve_class" => super::signup_confirm::<PvPData>(interaction, ctx, None).await,
        "tank_class" => super::signup_confirm::<PvPData>(interaction, ctx, Some(EventRole::Tank)).await,
        "ganker_class" => super::signup_confirm::<PvPData>(interaction, ctx, Some(EventRole::Ganker)).await,
        "healer_class" => super::signup_confirm::<PvPData>(interaction, ctx, Some(EventRole::Healer)).await,
        "brawler_class" => super::signup_confirm::<PvPData>(interaction, ctx, Some(EventRole::Brawler)).await,
        "bomber_class" => super::signup_confirm::<PvPData>(interaction, ctx, Some(EventRole::Bomber)).await,
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}