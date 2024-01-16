mod trials;
mod pvp;
mod generic;

use std::fmt::Display;
use std::str::FromStr;
use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage, Mention, Message};
use serenity::builder::CreateMessage;
use crate::events::components::{select_flex_roles, select_player_class};
use crate::events::models::{EventBasicData, EventEmbed, EventKind, EventRole, Player};
use crate::events::signup::{EventBackupRoles, EventSignupRoles};
use crate::prelude::*;

const PREFIX: &'static str = "signup_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<()> {
    let response = if interaction.data.custom_id.starts_with(&format!("{}trial", PREFIX)) {
        Ok(trials::handle_component(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}pvp", PREFIX)) {
        Ok(pvp::handle_component(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}generic", PREFIX)) {
        Ok(generic::handle_component(interaction, ctx)?)
    } else if interaction.data.custom_id == format!("{}absent", PREFIX) {
        let mut data = EventKind::try_from(*interaction.message.clone()).unwrap();
        data.add_absent(interaction.user.id);
        Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new().embed(data.get_embed())))
    } else {
        Err(Error::UnknownInteraction(PREFIX.to_string()))
    }?;

    interaction.create_response(&ctx.http, response).await?;

    Ok(())
}

fn signup_class_flex(id: impl Into<String>+Display, flex_roles: &[EventRole]) -> Result<CreateInteractionResponse> {
    let class_selector = select_player_class(format!("{}_class", id));
    let flex_selector = select_flex_roles(format!("{}_flex", id), flex_roles);

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .components(vec![flex_selector, class_selector])
    ))
}
async fn signup_confirm<T: TryFrom<Message>+EventSignupRoles+EventBackupRoles+EventEmbed+EventBasicData>(interaction: &ComponentInteraction, ctx: &Context, role: Option<EventRole>) -> Result<CreateInteractionResponse> {
    let selected_class = get_selected_option(interaction).unwrap();
    let mut flex_roles = interaction.message.embeds.first().map(|e| e
        .description.clone().unwrap()
        .split(",")
        .filter_map(|f| EventRole::from_str(f).ok())
        .collect()).unwrap_or(vec![]);

    let reference = interaction.message.message_reference.clone().unwrap();
    let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;

    if let Ok(mut event) = T::try_from(original_msg.clone()) {
        let dm = event.leader().create_dm_channel(&ctx.http).await?;
        if let Some(role) = role {
            if event.is_role_full(role) {
                if !flex_roles.contains(&role) {
                    flex_roles.push(role);
                }
                event.add_reserve(Player::Class(interaction.user.id, selected_class, flex_roles))
            } else {
                let user = Mention::User(interaction.user.id).to_string();
                let channel = Mention::Channel(interaction.channel_id).to_string();
                let flex = flex_roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();
                dm.send_message(&ctx.http, CreateMessage::new()
                    .content(format!("{user} se ha apuntado al evento en {channel} como {}, y flexible a: {}", role, flex.join(",")))
                ).await?;
                event.signup(role, Player::Class(interaction.user.id, selected_class, flex_roles));
            }
        } else {
            event.add_reserve(Player::Class(interaction.user.id, selected_class, flex_roles))
        }

        original_msg.edit(&ctx.http, EditMessage::new().embed(event.get_embed())).await?;
    }

    Ok(CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().description("Ya estas dentro!"))
            .components(vec![])
    ))
}

fn update_flex_preview(interaction: &ComponentInteraction) -> Result<CreateInteractionResponse> {
    let selected_flex = get_selected_options(interaction);

    let response = if selected_flex.is_empty() {
        CreateInteractionResponseMessage::new()
    } else {
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().title("Roles de reserva").description(selected_flex.join(",")))
    };

    Ok(CreateInteractionResponse::UpdateMessage(response))
}

fn get_selected_option(interaction: &ComponentInteraction) -> Option<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        Some(values.first().unwrap().to_string())
    } else { None }
}

fn get_selected_options(interaction: &ComponentInteraction) -> Vec<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
}