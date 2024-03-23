use serenity::all::{ButtonStyle, CommandInteraction, ComponentInteraction, Context, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, Message};
use crate::events::EventKind;
use crate::prelude::*;

pub(super) async fn select_event_kind(interaction: &CommandInteraction, ctx: &Context) -> Result<(ComponentInteraction, EventKind, Message)> {
    interaction.create_response(&ctx.http, create_event_message()).await?;
    let create_event_msg = interaction.get_response(&ctx.http).await?;

    if let Some(interaction) = create_event_msg.await_component_interaction(&ctx.shard).await {
        if let Some(kind) = EventKind::from_partial_id(&interaction.data.custom_id) {
            return Ok((interaction, kind, create_event_msg))
        }
    }

    Err(Error::Timeout)
}

fn create_event_message() -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(CreateEmbed::new()
                       .title("Nuevo evento")
                       .description("Elige tipo de evento"))
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("create_event_trial")
                    .label("Trial")
                    .style(ButtonStyle::Secondary),
                CreateButton::new("create_event_pvp")
                    .label("PvP")
                    .style(ButtonStyle::Secondary)
            ])]))
}