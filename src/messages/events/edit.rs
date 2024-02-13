use std::time::Duration;
use serenity::all::{ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, EditMessage};
use crate::events::{Event, EventRole, Player};
use crate::prelude::*;
use serenity::futures::StreamExt;

pub async fn edit_event(interaction: &CommandInteraction, ctx: &Context, store: &Store) -> Result<()> {
    let mut message = interaction.data.resolved.messages.values().next().unwrap().clone();

    if let Ok(event) = store.get_event(message.id).await {
        interaction.create_response(&ctx.http, edit_event_message(event)).await?;
        let preview_message = interaction.get_response(&ctx.http).await?;

        let mut modification = preview_message
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(60 * 3)).stream();

        while let Some(interaction) = modification.next().await {
            if let Some(role) = EventRole::from_partial_id(&interaction.data.custom_id) {
                let response = CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .components(vec![
                            CreateActionRow::SelectMenu(CreateSelectMenu::new(
                                format!("edit_role_select_{role}"),
                                CreateSelectMenuKind::User { default_users: None})
                            )
                        ])
                );

                interaction.create_response(&ctx.http, response).await?;
                let response = preview_message.await_component_interaction(&ctx.shard)
                    .timeout(Duration::from_secs(60 * 3)).await;
                if let Some(interaction) = response {
                    let users = get_selected_users(&interaction);
                    let guild = interaction.guild_id.clone().unwrap();
                    for user in users {
                        let member = guild.member(&ctx.http, user).await?;
                        store.signup_player(message.id, role, Player::new(user, member.display_name().to_string())).await?;
                    }
                    let event = store.get_event(message.id).await?;
                    message.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;
                }
            }

            // edit fields
        }
    } else {
        interaction.create_response(&ctx.http, super::not_an_event_response()).await?;
    }

    Ok(())
}

fn edit_event_message(event: Event) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .components(vec![
            CreateActionRow::Buttons(event.roles.iter()
                .filter_map(|pr| if pr.role.is_backup_role() {None} else { Some(edit_event_button(&pr.role))}).collect()),
            CreateActionRow::Buttons(event.roles.iter()
                .filter_map(|pr| if !pr.role.is_backup_role() {None} else { Some(edit_event_button(&pr.role)
                    .style(ButtonStyle::Secondary))}).collect()),

        ]))
}

fn edit_event_button(role: &EventRole) -> CreateButton {
    CreateButton::new(format!("edit_role_{}", role.to_id()))
        .label(format!("Mover a {role}"))
        .emoji(role.emoji())
        .style(ButtonStyle::Success)
}