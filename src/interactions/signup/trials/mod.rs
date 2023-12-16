use serenity::all::{ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage};
use crate::events::components::select_player_class;
use crate::events::signup::EventSignupRoles;
use crate::events::trials::embeds::trial_embed;
use crate::events::trials::models::TrialData;
use crate::events::trials::TrialRole;
use crate::prelude::*;

const PREFIX: &'static str = "trial_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "tank" => signup_trial(interaction, TrialRole::Tank, "tank_class"),
        "dd" => signup_trial(interaction, TrialRole::DD, "dd_class"),
        "healer" => signup_trial(interaction, TrialRole::Healer, "healer_class"),
        "healer_class" => signup_trial_class(&interaction, ctx, TrialRole::Healer).await,
        "dd_class" => signup_trial_class(&interaction, ctx, TrialRole::DD).await,
        "tank_class" => signup_trial_class(&interaction, ctx, TrialRole::Tank).await,
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn signup_trial(interaction: &ComponentInteraction, role: TrialRole, class_selector: &str) -> Result<CreateInteractionResponse> {
    let data = TrialData::try_from(*interaction.message.clone())?;

    if data.is_role_full(role) {
        Err(Error::RoleFull(format!("{:?}", role)))
    } else {
        let class_selector = select_player_class(&format!("{}{}{}", super::PREFIX, PREFIX, class_selector));
        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .components(class_selector)
        ))
    }
}

async fn signup_trial_class(interaction: &ComponentInteraction, ctx: &Context, role: TrialRole) -> Result<CreateInteractionResponse> {
    let selected_class = super::get_selected_class(interaction).unwrap();
    let reference = interaction.message.message_reference.clone().unwrap();
    let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;
    let mut trial = TrialData::try_from(original_msg.clone()).unwrap();
    trial.signup_class(role, interaction.user.id, selected_class);
    original_msg.edit(&ctx.http, EditMessage::new().embed(trial_embed(&trial, false))).await?;
    Ok(CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().description("Ya estas dentro!"))
            .components(vec![])
    ))
}