use serenity::all::{CreateInteractionResponse, ModalInteraction};
use serenity::client::Context;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
    let msg = events::trials::select_date("event_days", &interaction.data.components, &interaction.user.id)?;
    interaction.create_response(&ctx.http,CreateInteractionResponse::UpdateMessage(msg)).await?;
    Ok(())
}

