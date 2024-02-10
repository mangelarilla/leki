use serenity::all::{ChannelId, CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, GetMessages, Message, MessageType};
use crate::messages::BotInteractionMessage;
use crate::prelude::*;

pub struct DeleteEvent;

impl DeleteEvent {
    pub fn new() -> Self {
        DeleteEvent
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for DeleteEvent {
    async fn command(&self, interaction: &CommandInteraction, ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let message = interaction.data.resolved.messages.keys().next().unwrap();

        if let Ok(event) = store.get_event(*message).await {
            let channel_messages = get_event_channel_messages(interaction.channel_id, ctx).await?;

            interaction.channel_id.delete_messages(&ctx.http, channel_messages).await?;
            store.remove_event(*message).await?;

            if let Some(id) = event.scheduled_event {
                let guild = interaction.guild_id.unwrap();
                guild.delete_scheduled_event(&ctx.http, id).await?;
            }

            Ok(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("Purgado!")
            ))
        } else {
            Ok(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .content("Eso no es un evento atontao!")
                .ephemeral(true)
            ))
        }
    }
}

async fn get_event_channel_messages(channel: ChannelId, ctx: &Context) -> Result<Vec<Message>> {
    let messages = channel.messages(&ctx.http, GetMessages::new()).await?
        .into_iter()
        .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
        .collect::<Vec<Message>>();
    Ok(messages)
}