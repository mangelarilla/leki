use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, ModalInteraction};
use crate::error::Error;
use crate::events::components::{select_event_channel};
use crate::events::models::{EventBasicData, EventRole};
use crate::events::pvp::components::{pvp_basic_info, pvp_participants_components};
use crate::events::pvp::embeds::pvp_embed;
use crate::events::pvp::models::PvPData;

const PREFIX: &'static str = "pvp_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> crate::prelude::Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = if event_id.starts_with("times") {
        Ok(super::create_event(interaction, ctx, true).await?)
    } else {
        match event_id.as_str() {
            "event" => Ok(request_basic_pvp_data()),
            "public" => Ok(request_day_channel()),
            "semi_public" => Ok(request_semi_public_roster_choices()),
            "private" => Ok(request_private_roster_choices()),
            "semi_public_tanks" | "private_tanks" => Ok(super::update_preview_with_role::<PvPData>(interaction, EventRole::Tank)),
            "semi_public_brawlers" | "private_brawlers" => Ok(super::update_preview_with_role::<PvPData>(interaction, EventRole::Brawler)),
            "semi_public_healers" | "private_healers" => Ok(super::update_preview_with_role::<PvPData>(interaction, EventRole::Healer)),
            "semi_public_bombers" | "private_bombers" => Ok(super::update_preview_with_role::<PvPData>(interaction, EventRole::Bomber)),
            "semi_public_gankers" | "private_gankers" => Ok(super::update_preview_with_role::<PvPData>(interaction, EventRole::Ganker)),
            "semi_public_confirm" => Ok(request_day_channel()),
            "private_confirm" => Ok(request_day_channel_with_private_roster(interaction)),
            "event_day" => Ok(super::request_event_times(&prefixed("times"), ctx, interaction).await?),
            "comp_confirm" => Ok(super::request_event_scope(interaction, prefixed("public"), prefixed("semi_public"), prefixed("private"))),
            "comp_change" => Ok(super::request_new_comp::<PvPData>(prefixed("comp_new"))),
            _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
        }
    }?;

    Ok(response)
}

pub(super) async fn handle_modal(interaction: &ModalInteraction, _ctx: &Context) -> crate::prelude::Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "basic_info" => Ok(super::request_event_comp_and_create_preview::<PvPData>(interaction, prefixed("comp_confirm"), prefixed("comp_change"))),
        "comp_new" => Ok(super::update_preview_and_request_event_scope::<PvPData>(interaction, prefixed("public"), prefixed("semi_public"), prefixed("private"))),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}

fn request_day_channel_with_private_roster(interaction: &ComponentInteraction) -> CreateInteractionResponse {
    let pvp = PvPData::try_from(*interaction.message.clone()).unwrap();
    let response = CreateInteractionResponseMessage::new()
        .embeds(vec![pvp_embed(&pvp, true)
            .title(format!("[Roster Cerrado] {}", pvp.title()))
        ])
        .components(select_event_channel(&prefixed("event_day")));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_day_channel() -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .components(select_event_channel(&prefixed("event_day")));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_semi_public_roster_choices() -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .components(pvp_participants_components(
            &prefixed("semi_public_tanks"), &prefixed("semi_public_brawlers"), &prefixed("semi_public_healers"),
            &prefixed("semi_public_bombers"), &prefixed("semi_public_gankers"), &prefixed("semi_public_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_private_roster_choices() -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .components(pvp_participants_components(
            &prefixed("private_tanks"), &prefixed("private_brawlers"), &prefixed("private_healers"),
            &prefixed("private_bombers"), &prefixed("private_gankers"), &prefixed("private_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_basic_pvp_data() -> CreateInteractionResponse {
    let response = CreateModal::new(&prefixed("basic_info"), "Informacion del Evento")
        .components(pvp_basic_info());

    CreateInteractionResponse::Modal(response)
}