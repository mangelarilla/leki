use serenity::all::{ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage};
use crate::error::Error;
use crate::events::components::select_player_class;
use crate::events::pvp::embeds::pvp_embed;
use crate::events::pvp::models::PvPData;
use crate::events::pvp::PvPRole;
use crate::events::signup::EventSignupRoles;
use crate::prelude::*;

const PREFIX: &'static str = "pvp_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
    let event_id = interaction.data.custom_id
        .replace(super::PREFIX, "").replace(PREFIX, "");

    let response = match event_id.as_str() {
        "tank" => signup_pvp(interaction, PvPRole::Tank, "tank_class"),
        "brawler" => signup_pvp(interaction, PvPRole::Brawler, "brawler_class"),
        "healer" => signup_pvp(interaction, PvPRole::Healer, "healer_class"),
        "bomber" => signup_pvp(interaction, PvPRole::Bomber, "bomber_class"),
        "ganker" => signup_pvp(interaction, PvPRole::Ganker, "ganker_class"),
        "healer_class" => signup_pvp_class(&interaction, ctx, PvPRole::Healer).await,
        "brawler_class" => signup_pvp_class(&interaction, ctx, PvPRole::Brawler).await,
        "tank_class" => signup_pvp_class(&interaction, ctx, PvPRole::Tank).await,
        "bomber_class" => signup_pvp_class(&interaction, ctx, PvPRole::Bomber).await,
        "ganker_class" => signup_pvp_class(&interaction, ctx, PvPRole::Ganker).await,
        _ => Err(Error::UnknownInteraction(interaction.data.custom_id.to_string()))
    }?;

    Ok(response)
}

fn signup_pvp(interaction: &ComponentInteraction, role: PvPRole, class_selector: &str) -> Result<CreateInteractionResponse> {
    let pvp = PvPData::try_from(*interaction.message.clone())?;

    if pvp.is_role_full(role) {
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

async fn signup_pvp_class(interaction: &ComponentInteraction, ctx: &Context, role: PvPRole) -> Result<CreateInteractionResponse> {
    let selected_class = super::get_selected_class(interaction).unwrap();
    let reference = interaction.message.message_reference.clone().unwrap();
    let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;
    let mut pvp = PvPData::try_from(original_msg.clone()).unwrap();
    pvp.signup_class(role, interaction.user.id, selected_class);
    original_msg.edit(&ctx.http, EditMessage::new().embed(pvp_embed(&pvp, false))).await?;
    Ok(CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().description("Ya estas dentro!"))
            .components(vec![])
    ))
}