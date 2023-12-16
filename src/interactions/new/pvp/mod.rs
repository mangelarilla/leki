use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, ModalInteraction};
use crate::error::Error;
use crate::events::components::{event_scope_components, select_event_channel};
use crate::events::models::EventBasicData;
use crate::events::pvp::components::{pvp_basic_info, pvp_participants_components};
use crate::events::pvp::embeds::pvp_embed;
use crate::events::pvp::models::PvPData;
use crate::events::pvp::PvPRole;
use crate::events::signup::EventSignupRoles;
use crate::interactions::new::{create_event, get_selected_users, request_event_times};

const PREFIX: &'static str = "pvp_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> crate::prelude::Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "event" => Ok(request_basic_pvp_data()),
        "public" => Ok(request_day_channel()),
        "semi_public" => Ok(request_semi_public_roster_choices()),
        "private" => Ok(request_private_roster_choices()),
        "semi_public_tanks" | "private_tanks" => Ok(update_preview_with_role(interaction, PvPRole::Tank)),
        "semi_public_brawlers" | "private_brawlers" => Ok(update_preview_with_role(interaction, PvPRole::Brawler)),
        "semi_public_healers" | "private_healers" => Ok(update_preview_with_role(interaction, PvPRole::Healer)),
        "semi_public_bombers" | "private_bombers" => Ok(update_preview_with_role(interaction, PvPRole::Bomber)),
        "semi_public_confirm" => Ok(request_day_channel()),
        "private_confirm" => Ok(request_day_channel_with_private_roster(interaction)),
        "event_day" => Ok(request_event_times(&prefixed("times"), ctx, interaction).await?),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

pub(super) async fn handle_modal(interaction: &ModalInteraction, ctx: &Context) -> crate::prelude::Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "basic_info" => Ok(request_pvp_scope_and_create_preview(interaction)),
        "times" => Ok(create_event(interaction, ctx, true).await?),
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn prefixed(id: &str) -> String {
    format!("{}{}{}", super::PREFIX, PREFIX, id)
}

fn update_preview_with_role(interaction: &ComponentInteraction, role: PvPRole) -> CreateInteractionResponse {
    let selected_users = get_selected_users(interaction);
    let response = if let Some(users) = selected_users {
        let mut pvp = PvPData::try_from(*interaction.message.clone()).unwrap();
        for user in users {
            pvp.signup(role, user);
        }
        CreateInteractionResponseMessage::new()
            .embeds(vec![pvp_embed(&pvp, true)])
    } else {
        CreateInteractionResponseMessage::new()
    };

    CreateInteractionResponse::UpdateMessage(response)
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
            &prefixed("semi_public_bombers"), &prefixed("semi_public_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_private_roster_choices() -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .components(pvp_participants_components(
            &prefixed("private_tanks"), &prefixed("private_brawlers"), &prefixed("private_healers"),
            &prefixed("private_bombers"), &prefixed("private_confirm")
        ));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_pvp_scope_and_create_preview(interaction: &ModalInteraction) -> CreateInteractionResponse {
    let pvp = PvPData::from_basic_modal(&interaction.data.components, interaction.user.id);
    let response = CreateInteractionResponseMessage::new()
        .add_embed(pvp_embed(&pvp, true))
        .components(event_scope_components(&prefixed("public"), &prefixed("semi_public"), &prefixed("private")));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_basic_pvp_data() -> CreateInteractionResponse {
    let response = CreateModal::new(&prefixed("basic_info"), "Informacion del Evento")
        .components(pvp_basic_info());

    CreateInteractionResponse::Modal(response)
}