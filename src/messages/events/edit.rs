mod roles;
mod info;

use std::time::Duration;
use serenity::all::{ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage, EmojiId, ReactionType};
use crate::events::{Event, EventRole};
use crate::prelude::*;
use serenity::futures::StreamExt;

pub async fn edit_event(interaction: &CommandInteraction, ctx: &Context, store: &Store) -> Result<()> {
    let mut message = interaction.data.resolved.messages.values().next().unwrap().clone();

    if let Ok(mut event) = store.get_event(message.id).await {
        interaction.create_response(&ctx.http, CreateInteractionResponse::Message(edit_event_message(&event))).await?;
        let preview_message = interaction.get_response(&ctx.http).await?;

        let mut modification = preview_message
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(60 * 5))
            .stream();

        while let Some(interaction) = modification.next().await {
            if let Some(role) = EventRole::from_partial_id(&interaction.data.custom_id) {
                if let Ok(interaction) = roles::edit_role(&interaction, ctx, store, role, message.id).await {
                    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(edit_event_message(&event))).await?;
                }
            }

            if interaction.data.custom_id == "edit_leader" {
                store.update_leader(message.id, interaction.user.id).await?;
                interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(edit_event_message(&event))).await?;
            }

            if interaction.data.custom_id == "edit_datetime" {
                if let Ok(modal) =info::edit_datetime(&interaction, ctx, store, &event, message.id).await {
                    modal.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(edit_event_message(&event))).await?;
                }
            }

            if interaction.data.custom_id == "edit_info" {
                if let Ok(modal) =info::edit_info(&interaction, ctx, store, &event, message.id).await {
                    modal.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(edit_event_message(&event))).await?;
                }
            }

            event = store.get_event(message.id).await?;
            message.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;
        }
    } else {
        interaction.create_response(&ctx.http, super::not_an_event_response()).await?;
    }

    Ok(())
}

fn edit_event_message(event: &Event) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .components(vec![
            // CreateActionRow::Buttons(event.roles.iter()
            //     .filter_map(|pr| if pr.role.is_backup_role() {None} else { Some(edit_event_button(&pr.role))}).collect()),
            CreateActionRow::Buttons(event.roles.iter()
                .filter_map(|pr| if !pr.role.is_backup_role() {None} else { Some(edit_event_button(&pr.role)
                    .style(ButtonStyle::Secondary))}).collect()),
            CreateActionRow::Buttons(vec![
                CreateButton::new("edit_leader").label("Robar evento").emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1138123617482322031), name: Some("potion".to_string()) }),
                CreateButton::new("edit_datetime").label("Cambiar Fecha y Hora").emoji(ReactionType::Unicode("📅".to_string())),
                CreateButton::new("edit_info").label("Cambiar info (titulo, descripcion...)").emoji(ReactionType::Unicode("ℹ️".to_string()))
            ])
        ])
}

fn edit_event_button(role: &EventRole) -> CreateButton {
    CreateButton::new(format!("edit_role_{}", role.to_id()))
        .label(format!("Mover a {role}"))
        .emoji(role.emoji())
        .style(ButtonStyle::Success)
}