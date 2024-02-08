use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, EditMessage, MessageId};
use serenity::builder::CreateSelectMenu;
use shuttle_persist::PersistInstance;
use crate::events::{Event, EventRole, Player};
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
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &PersistInstance) -> Result<CreateInteractionResponse> {
        let (_, msg_id) = interaction.data.custom_id.split_once("__")
            .unwrap();

        if interaction.data.custom_id == self.select_id() {
            let users = get_selected_users(interaction);

            let mut event = store.load::<Event>(msg_id)?;
            let guild = interaction.guild_id.clone().unwrap();
            for user in users {
                let member = guild.member(&ctx.http, user).await?;
                event.signup(self.role, Player::new(user, member.nick.unwrap()))
            }

            let msg_id = MessageId::new(msg_id.parse()?);
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