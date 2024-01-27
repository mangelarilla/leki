use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, MessageId};
use crate::messages::{BotInteractionFromCommandMessageAsync};
use crate::prelude::parse_event_link;

pub struct DeleteEvent;

impl DeleteEvent {
    pub fn new() -> Self {
        DeleteEvent
    }
}

#[async_trait]
impl BotInteractionFromCommandMessageAsync for DeleteEvent {
    async fn message(&self, interaction: &CommandInteraction, ctx: &Context) -> crate::prelude::Result<CreateInteractionResponse> {
        let message = interaction.data.resolved.messages.values().next().unwrap();
        if message.author.id != 1148032756899643412 {
            Ok(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .content("Eso no es un evento atontao!")
                .ephemeral(true)
            ))
        } else {
            let channel_messages = crate::interactions::get_event_channel_messages(message.channel_id, ctx).await.unwrap();
            message.channel_id.delete_messages(&ctx.http, channel_messages).await.unwrap();

            let guild = interaction.guild_id.unwrap();
            let events = guild.scheduled_events(&ctx.http, false).await.unwrap();
            for event in events {
                if event.creator_id.unwrap() == 1148032756899643412 {
                    let (_, _, event_msg) = parse_event_link(&event.description.unwrap());
                    if MessageId::new(event_msg) == message.id {
                        guild.delete_scheduled_event(&ctx.http, event.id).await.unwrap();
                        crate::tasks::unset_reminder(&message.channel_id);
                    }
                }
            }

            Ok(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("Purgado!")
            ))
        }
    }
}