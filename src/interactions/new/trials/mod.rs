use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, ModalInteraction};
use crate::events::components::{event_scope_components, select_event_channel};
use crate::events::models::EventBasicData;
use crate::events::signup::EventSignupRoles;
use crate::events::trials::components::{trial_basic_info_components, trial_participants_components};
use crate::events::trials::embeds::{trial_embed};
use crate::events::trials::models::TrialData;
use crate::events::trials::TrialRole;
use crate::interactions::new::{create_event, get_selected_users, request_event_times};
use crate::prelude::*;

const PREFIX: &'static str = "trial_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "event" => Ok(request_basic_trial_data()),
        "public" => Ok(request_day_channel()),
        "semi_public" => Ok(request_semi_public_roster_choices()),
        "private" => Ok(request_private_roster_choices()),
        "semi_public_tanks" | "private_tanks" => Ok(update_preview_with_role(interaction, TrialRole::Tank)),
        "semi_public_dd" | "private_dd" => Ok(update_preview_with_role(interaction, TrialRole::DD)),
        "semi_public_healers" | "private_healers" => Ok(update_preview_with_role(interaction, TrialRole::Healer)),
        "semi_public_confirm" => Ok(request_day_channel()),
        "private_confirm" => Ok(request_day_channel_with_private_roster(interaction)),
        "event_day" => Ok(request_event_times(&prefixed("times"), ctx, interaction).await?),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

pub(super) async fn handle_modal(interaction: &ModalInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "basic_info" => Ok(request_trial_scope_and_create_preview(interaction)),
        "times" => Ok(create_event(interaction, ctx, false).await?),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}

fn update_preview_with_role(interaction: &ComponentInteraction, role: TrialRole) -> CreateInteractionResponse {
    let selected_users = get_selected_users(interaction);
    let response = if let Some(users) = selected_users {
        let mut trial = TrialData::try_from(*interaction.message.clone()).unwrap();
        for user in users {
            trial.signup(role, user);
        }
        CreateInteractionResponseMessage::new()
            .embeds(vec![trial_embed(&trial, true)])
    } else {
        CreateInteractionResponseMessage::new()
    };

    CreateInteractionResponse::UpdateMessage(response)
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
            &prefixed("semi_public_tanks"), &prefixed("semi_public_dd"), &prefixed("semi_public_healers"),
            &prefixed("semi_public_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_private_roster_choices() -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .components(trial_participants_components(
            &prefixed("private_tanks"), &prefixed("private_dd"), &prefixed("private_healers"),
            &prefixed("private_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_trial_scope_and_create_preview(interaction: &ModalInteraction) -> CreateInteractionResponse {
    let trial = TrialData::from_basic_modal(&interaction.data.components, interaction.user.id);
    let response = CreateInteractionResponseMessage::new()
        .add_embed(trial_embed(&trial, true))
        .components(event_scope_components(&prefixed("public"), &prefixed("semi_public"), &prefixed("private")));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_basic_trial_data() -> CreateInteractionResponse {
    let response = CreateModal::new(&prefixed("basic_info"), "Informacion de la Trial")
        .components(trial_basic_info_components());

    CreateInteractionResponse::Modal(response)
}

