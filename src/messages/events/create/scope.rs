use std::time::Duration;
use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, Message};
use crate::events::{Event, EventRole, EventScopes, Player};
use crate::prelude::*;
use serenity::futures::StreamExt;

pub(super) async fn handle_scope(message: &Message, interaction: &ComponentInteraction, ctx: &Context, event: &mut Event) -> Result<ComponentInteraction> {
    // Select scope
    interaction.create_response(&ctx.http, create_event_scope_select(&event)).await?;
    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        event.scope = EventScopes::from_partial_id(&interaction.data.custom_id);

        if event.scope != EventScopes::Public {

            // Select role to add players
            interaction.create_response(&ctx.http, create_event_scope_role(event)).await?;

            let mut role_add_players = message
                .await_component_interaction(&ctx.shard)
                .timeout(Duration::from_secs(60 * 5))
                .stream();

            while let Some(interaction) = role_add_players.next().await {
                if let Some(role) = EventRole::from_partial_id(&interaction.data.custom_id) {
                    if !interaction.data.custom_id.ends_with("select") {

                        // Display select menu for role
                        interaction.create_response(&ctx.http, create_event_scope_role_select(role, event)).await?;
                    } else {

                        // Get nicknames and signup players
                        let guild = interaction.guild_id.clone().unwrap();
                        for user in get_selected_users(&interaction) {
                            let member = guild.member(&ctx.http, user).await?;
                            event.add_player(role, Player::new(user, member.display_name().to_string()));
                        }

                        interaction.create_response(&ctx.http, create_event_scope_role(event)).await?;
                    }
                } else {
                    return Ok(interaction);
                }
            }
        }

        Ok(interaction)
    } else {
        Err(Error::Timeout)
    }
}

fn create_event_scope_select(event: &Event) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .button(CreateButton::new("create_event_scope_public")
                .label("Abierto")
                .style(ButtonStyle::Success))
            .button(CreateButton::new("create_event_scope_semi_public")
                .label("Semi-abierto")
                .style(ButtonStyle::Secondary))
            .button(CreateButton::new("create_event_scope_private")
                .label("Cerrado")
                .style(ButtonStyle::Danger))
    )
}

fn create_event_scope_role(event: &Event) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .components(vec![scope_role_buttons(event), scope_reserve_button(), scope_confirm()])
    )
}

fn create_event_scope_role_select(role: EventRole, event: &Event) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .components(vec![
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(format!("create_event_scope_{}_select", role.to_id()), CreateSelectMenuKind::User {
                        default_users: None
                    }).max_values(12)
                ),
                scope_role_buttons(event),
                scope_reserve_button(),
                scope_confirm()
            ])
    )
}

fn scope_role_buttons(event: &Event) -> CreateActionRow {
    CreateActionRow::Buttons(event.kind.roles()
        .into_iter()
        .filter_map(|role| if !role.is_backup_role() {
            Some(role.to_button(format!("create_event_scope_{}", role.to_id()), role.to_string()))
        } else { None }).collect()
    )
}

fn scope_reserve_button() -> CreateActionRow {
    let role = EventRole::Reserve;
    CreateActionRow::Buttons(vec![
        role.to_button(format!("create_event_scope_{}", role.to_id()), role.to_string())
    ])
}

fn scope_confirm() -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        CreateButton::new("create_event_scope_confirm")
            .label("Continuar")
            .style(ButtonStyle::Secondary)
    ])
}