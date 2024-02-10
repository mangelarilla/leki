use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, EditMessage, MessageId};
use serenity::builder::CreateSelectMenu;
use crate::events::{EventRole, Player};
use crate::messages::{BotInteractionMessage};
use crate::messages::events::edit_event::edit_event;
use crate::prelude::*;

#[derive(Clone)]
pub struct EditEventRole {
    role: EventRole
}

impl EditEventRole {
    pub fn new(role: EventRole) -> Self {
        EditEventRole { role }
    }

    pub(super) fn select_id(&self) -> String {
        format!("edit_role_{}", self.role.to_id())
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for EditEventRole {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let (custom_id, msg_id) = interaction.data.custom_id.split_once("__")
            .unwrap();

        if custom_id == self.select_id() {
            let users = get_selected_users(interaction);
            let msg_id = MessageId::new(msg_id.parse()?);


            let guild = interaction.guild_id.clone().unwrap();
            for user in users {
                let member = guild.member(&ctx.http, user).await?;
                store.signup_player(msg_id, self.role, Player::new(user, member.display_name().to_string())).await?;
            }

            let event = store.get_event(msg_id).await?;
            let mut original_msg = interaction.channel_id.message(&ctx.http, msg_id).await?;

            original_msg.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;

            Ok(CreateInteractionResponse::UpdateMessage(edit_event(event, msg_id)))
        } else {
            let components = vec![
                CreateActionRow::SelectMenu(CreateSelectMenu::new(
                    format!("{}__{msg_id}", self.select_id()),
                    CreateSelectMenuKind::User { default_users: None})
                )
            ];

            Ok(CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .components(components)
            ))
        }
    }
}