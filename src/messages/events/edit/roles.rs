use std::time::Duration;
use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, MessageId};
use crate::events::{EventRole, Player};
use crate::prelude::*;

pub(super) async fn edit_role(interaction: &ComponentInteraction, ctx: &Context, store: &Store, role: EventRole, msg_id: MessageId) -> Result<ComponentInteraction> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .components(vec![
                CreateActionRow::SelectMenu(CreateSelectMenu::new(
                    format!("edit_role_select_{role}"),
                    CreateSelectMenuKind::User { default_users: None})
                )
            ])
    )).await?;
    let response = interaction.message.await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3)).await;
    if let Some(interaction) = response {
        let users = get_selected_users(&interaction);
        let guild = interaction.guild_id.clone().unwrap();
        for user in users {
            let member = guild.member(&ctx.http, user).await?;
            store.signup_player(msg_id, role, Player::new(user, member.display_name().to_string())).await?;
        }
        Ok(interaction)
    } else {
        Err(Error::Timeout)
    }
}