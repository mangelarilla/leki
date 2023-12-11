use serenity::all::{ComponentInteraction, CreateInteractionResponse};
use serenity::prelude::Context;

use crate::prelude::*;
pub(crate) async fn handle(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    interaction.create_response(
        &ctx.http,
        CreateInteractionResponse::Modal(events::trials::data("trial_texts"))
    ).await?;
    Ok(())
}