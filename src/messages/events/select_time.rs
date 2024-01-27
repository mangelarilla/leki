use async_trait::async_trait;
use crate::prelude::*;
use serenity::all::{ChannelId, ComponentInteraction, ComponentInteractionDataKind, Context, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::messages::{BotInteractionFromComponentMessageAsync};

pub(crate) struct SelectTime {
    time_id: String
}

impl SelectTime {
    pub(crate) fn new(time_id: impl Into<String>) -> Self {
        SelectTime {time_id: time_id.into()}
    }
}

#[async_trait]
impl BotInteractionFromComponentMessageAsync for SelectTime {
    async fn message(&self, interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
        let mut channels = vec![];
        for channel_id in get_selected_channels(interaction) {
            let name = channel_id.name(&ctx.http).await?;
            channels.push((channel_id.clone(), get_channel_weekday(&name).unwrap()));
        }

        Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
            .components(events::components::select_time(&self.time_id, &channels))
        ))
    }
}

fn get_selected_channels(interaction: &ComponentInteraction) -> Vec<ChannelId> {
    if let ComponentInteractionDataKind::ChannelSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
}