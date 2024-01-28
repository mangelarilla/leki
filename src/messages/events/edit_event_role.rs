use std::marker::PhantomData;
use async_trait::async_trait;
use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, EditMessage, GetMessages};
use serenity::builder::CreateSelectMenu;
use crate::events::{EventData, EventRole, Player};
use crate::messages::{BotInteractionFromComponentMessageAsync, BotInteractionMessage};
use crate::messages::events::edit_event::{edit_event};
use crate::prelude::*;

pub struct EditEventRole<T: EventData + 'static + Sync + Send> {
    role: EventRole,
    phantom: PhantomData<T>
}

impl<T: EventData + 'static + Sync + Send> EditEventRole<T> {
    pub fn new(role: EventRole) -> Self {
        EditEventRole { role, phantom: PhantomData }
    }
}

impl<T: EventData + 'static + Sync + Send> BotInteractionMessage for EditEventRole<T> {
    fn message(&self) -> Result<CreateInteractionResponse> {
        let components = vec![
            CreateActionRow::SelectMenu(CreateSelectMenu::new(
                T::prefix_id(format!("edit_role_{}", self.role.to_id())),
                CreateSelectMenuKind::User { default_users: None})
            )
        ];

        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .components(components)
        ))
    }
}

#[async_trait]
impl<T: EventData + 'static + Sync + Send> BotInteractionFromComponentMessageAsync for EditEventRole<T>
    where Error: From<<T as TryFrom<serenity::all::Message>>::Error>{
    async fn message(&self, interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
        let users = get_selected_users(interaction);

        let channel_messages = interaction.channel_id.messages(&ctx.http, GetMessages::new()
            .limit(5)).await?;
        let mut original_msg = channel_messages.into_iter()
            .find(|msg| !msg.pinned && msg.author.id == 1148032756899643412)// Leki id
            .unwrap();

        let mut event = T::try_from(original_msg.clone())?;
        if self.role == EventRole::Reserve {
            for user in users {
                event.add_reserve(Player::Basic(user));
            }
        } else if let EventRole::Signed(s) = self.role {
            for user in users {
                event.signup(s, Player::Basic(user));
            }
        }

        original_msg.edit(&ctx.http, EditMessage::new().embed(event.get_embed())).await?;

        Ok(CreateInteractionResponse::UpdateMessage(edit_event(event)))
    }
}