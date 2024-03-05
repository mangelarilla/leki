use std::time::Duration;
use serenity::all::{ButtonStyle, ChannelId, CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, GetMessages, Mention, Message, MessageId, MessageType, ScheduledEventStatus};
use serenity::builder::CreateButton;
use tracing::{info, instrument};
use crate::prelude::*;

#[instrument]
pub async fn delete_event(interaction: &CommandInteraction, ctx: &Context, store: &Store) -> Result<()> {
    let message = interaction.data.resolved.messages.keys().next().unwrap();
    info!("Event to delete: {message}");
    if let Ok(event) = store.get_event(*message).await {
        if let Some(id) = event.scheduled_event {
            let guild = interaction.guild_id.unwrap();

            // Scheduled event exists
            if guild.scheduled_event(&ctx.http, id, false).await.is_ok_and(|se| se.status == ScheduledEventStatus::Scheduled) {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content(format!(r#"
Este evento aun no se ha producido, __**SEGURO QUE QUIERES BORRARLO??**__

Estas bien {}?? Quieres hablar??
"#, Mention::User(interaction.user.id)))
                        .button(CreateButton::new("delete_event_confirm")
                            .label("Si, borralo, puta vida").style(ButtonStyle::Danger))
                )).await?;

                let response = interaction.get_response(&ctx.http).await?;
                if let Some(interaction) = response.await_component_interaction(&ctx.shard).timeout(Duration::from_secs(60)).await {
                    guild.delete_scheduled_event(&ctx.http, id).await?;
                    remove_event(store, ctx, interaction.channel_id, *message).await?;

                    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(purged().embeds(vec![]))).await?;
                }
            } else {
                remove_event(store, ctx, interaction.channel_id, *message).await?;
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(purged())).await?;
            }
        } else {
            remove_event(store, ctx, interaction.channel_id, *message).await?;
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(purged())).await?;
        }
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

async fn remove_event(store: &Store, ctx: &Context, channel: ChannelId, message: MessageId) -> Result<()> {
    let channel_messages = get_event_channel_messages(channel, ctx).await?;
    channel.delete_messages(&ctx.http, channel_messages).await?;
    store.remove_event(message).await?;

    Ok(())
}

fn purged() -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .content("Purgado!")
}