use serenity::all::{ChannelId, CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, GetMessages, Message, MessageType};
use crate::prelude::*;

pub async fn delete_event(interaction: &CommandInteraction, ctx: &Context, store: &Store) -> Result<()> {
    let message = interaction.data.resolved.messages.keys().next().unwrap();

    if let Ok(event) = store.get_event(*message).await {
        let channel_messages = get_event_channel_messages(interaction.channel_id, ctx).await?;

        interaction.channel_id.delete_messages(&ctx.http, channel_messages).await?;
        store.remove_event(*message).await?;

        if let Some(id) = event.scheduled_event {
            let guild = interaction.guild_id.unwrap();
            guild.delete_scheduled_event(&ctx.http, id).await?;
        }

        interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .content("Purgado!")
        )).await?;
    } else {
        interaction.create_response(&ctx.http, super::not_an_event_response()).await?;
    }

    Ok(())
}

async fn get_event_channel_messages(channel: ChannelId, ctx: &Context) -> Result<Vec<Message>> {
    let messages = channel.messages(&ctx.http, GetMessages::new()).await?
        .into_iter()
        .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
        .collect::<Vec<Message>>();
    Ok(messages)
}