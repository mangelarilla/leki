use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, CreateInteractionResponse};
use serenity::client::Context;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
        let response = events::trials::select_time("event_dates", values);
        interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(response)).await?;
    }
    Ok(())
}