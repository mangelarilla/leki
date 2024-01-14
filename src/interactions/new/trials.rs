use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, ModalInteraction};
use crate::events::components::{select_event_channel};
use crate::events::models::{EventBasicData, EventEmbed, EventRole, Player};
use crate::events::signup::EventBackupRoles;
use crate::events::trials::components::{trial_basic_info_components, trial_participants_components};
use crate::events::trials::embeds::{trial_embed};
use crate::events::trials::models::TrialData;
use crate::prelude::*;

const PREFIX: &'static str = "trial_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = if event_id.starts_with("times") {
        Ok(super::create_event(interaction, ctx, false).await?)
    } else {
        match event_id.as_str() {
            "event" => Ok(request_basic_trial_data()),
            "public" => Ok(request_day_channel()),
            "semi_public" => Ok(request_semi_public_roster_choices()),
            "private" => Ok(request_private_roster_choices()),
            "semi_public_tanks" | "private_tanks" => Ok(super::update_preview_with_role::<TrialData>(interaction, EventRole::Tank)),
            "semi_public_dd" | "private_dd" => Ok(super::update_preview_with_role::<TrialData>(interaction, EventRole::DD)),
            "semi_public_healers" | "private_healers" => Ok(super::update_preview_with_role::<TrialData>(interaction, EventRole::Healer)),
            "semi_public_reserves" | "private_reserves" => Ok(update_preview_with_reserves(interaction)),
            "semi_public_confirm" => Ok(request_day_channel()),
            "private_confirm" => Ok(request_day_channel_with_private_roster(interaction)),
            "event_day" => Ok(super::request_event_times(&prefixed("times"), ctx, interaction).await?),
            "comp_confirm" => Ok(super::request_event_scope(interaction, prefixed("public"), prefixed("semi_public"), prefixed("private"))),
            "comp_change" => Ok(super::request_new_comp::<TrialData>(prefixed("comp_new"))),
            _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
        }
    }?;

    Ok(response)
}

pub(super) async fn handle_modal(interaction: &ModalInteraction, _ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "basic_info" => Ok(super::request_event_comp_and_create_preview::<TrialData>(interaction, prefixed("comp_confirm"), prefixed("comp_change"))),
        "comp_new" => Ok(super::update_preview_and_request_event_scope::<TrialData>(interaction, prefixed("public"), prefixed("semi_public"), prefixed("private"))),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}

fn request_day_channel_with_private_roster(interaction: &ComponentInteraction) -> CreateInteractionResponse {
    let trial = TrialData::try_from(*interaction.message.clone()).unwrap();
    let response = CreateInteractionResponseMessage::new()
        .embeds(vec![trial_embed(&trial, true)
            .title(format!("[Roster Cerrado] {}", trial.title()))
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
        .components(trial_participants_components(
            &prefixed("semi_public_tanks"),
            &prefixed("semi_public_dd"),
            &prefixed("semi_public_healers"),
            &prefixed("semi_public_reserves"),
            &prefixed("semi_public_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_private_roster_choices() -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .components(trial_participants_components(
            &prefixed("private_tanks"),
            &prefixed("private_dd"),
            &prefixed("private_healers"),
            &prefixed("private_reserves"),
            &prefixed("private_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_basic_trial_data() -> CreateInteractionResponse {
    let response = CreateModal::new(&prefixed("basic_info"), "Informacion de la Trial")
        .components(trial_basic_info_components());

    CreateInteractionResponse::Modal(response)
}

fn update_preview_with_reserves(interaction: &ComponentInteraction) -> CreateInteractionResponse {
    let selected_users = super::get_selected_users(interaction);

    let response = if let Some(users) = selected_users {
        if let Ok(mut event) = TrialData::try_from(*interaction.message.clone()) {
            for user in users {
                event.add_reserve(Player::Basic(user))
            }
            CreateInteractionResponseMessage::new()
                .embed(event.get_embed_preview())
        } else { CreateInteractionResponseMessage::new() }
    } else {
        CreateInteractionResponseMessage::new()
    };

    CreateInteractionResponse::UpdateMessage(response)
}

