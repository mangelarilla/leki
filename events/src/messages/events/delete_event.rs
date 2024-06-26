use std::time::Duration;
use serenity::all::{ButtonStyle, ChannelId, CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, GetMessages, Mention, Message, MessageId, MessageType, PartialChannel, ScheduledEventStatus};
use serenity::builder::CreateButton;
use sqlx::PgPool;
use tracing::{instrument};
use crate::prelude::*;

#[instrument]
pub async fn delete_event(interaction: &CommandInteraction, ctx: &Context, pool: PgPool) -> Result<()> {
    let store = Store::new(pool);
    let message = interaction.data.resolved.messages.values().next().unwrap();
    if let Ok(event) = store.get_event(message.id).await {
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
                    remove_event(&store, ctx, interaction.channel_id, message.id).await?;

                    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(purged().embeds(vec![]))).await?;
                }
            } else {
                remove_event(&store, ctx, interaction.channel_id, message.id).await?;
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(purged())).await?;
            }
        } else {
            remove_event(&store, ctx, interaction.channel_id, message.id).await?;
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(purged())).await?;
        }
    } else if message.author.id.get() == 1148032756899643412 && is_event_channel(interaction.channel.clone().unwrap()) {
        purge_channel(ctx, interaction.channel_id).await?;
    } else {
        interaction.create_response(&ctx.http, super::not_an_event_response()).await?;
    }

    Ok(())
}

fn is_event_channel(channel: PartialChannel) -> bool {
    let name = channel.name.unwrap();

    name.ends_with("-a") || name.ends_with("-b") || name.ends_with("-c")
}

async fn remove_event(store: &Store, ctx: &Context, channel: ChannelId, message: MessageId) -> Result<()> {
    purge_channel(ctx, channel).await?;
    store.remove_event(message).await?;

    Ok(())
}

async fn purge_channel(ctx: &Context, channel: ChannelId) -> Result<()> {
    let channel_messages = channel.messages(&ctx.http, GetMessages::new()).await?
        .into_iter()
        .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
        .collect::<Vec<Message>>();

    channel.delete_messages(&ctx.http, channel_messages).await?;

    Ok(())
}

fn purged() -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .content("Purgado!")
}