use serenity::all::{CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateEmbed;
use crate::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {

    let response = CreateInteractionResponseMessage::new()
        .embed(CreateEmbed::new().title("Nuevo evento").description("Elige tipo de evento"))
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("create_trial").label("Trial").style(ButtonStyle::Secondary),
            CreateButton::new("create_pvp").label("PvP").style(ButtonStyle::Secondary),
            CreateButton::new("create_generic").label("Generico").style(ButtonStyle::Secondary)
        ])]);

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(response)).await?;
    Ok(())
}