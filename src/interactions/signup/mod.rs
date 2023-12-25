mod trials;
mod pvp;
mod generic;

use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage};
use crate::events::models::{EventEmbed, EventKind};
use crate::events::signup::EventBackupRoles;
use crate::prelude::*;

const PREFIX: &'static str = "signup_";

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<()> {
    let response = if interaction.data.custom_id.starts_with(&format!("{}trial", PREFIX)) {
        Ok(trials::handle_component(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}pvp", PREFIX)) {
        Ok(pvp::handle_component(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}generic", PREFIX)) {
        Ok(generic::handle_component(interaction, ctx)?)
    } else if interaction.data.custom_id == format!("{}reserve", PREFIX) {
        let mut data = EventKind::try_from(*interaction.message.clone()).unwrap();
        data.add_reserve(interaction.user.id);
        Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new().embed(data.get_embed())))
    } else if interaction.data.custom_id == format!("{}absent", PREFIX) {
        let mut data = EventKind::try_from(*interaction.message.clone()).unwrap();
        data.add_absent(interaction.user.id);
        Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new().embed(data.get_embed())))
    } else {
        Err(Error::UnknownInteraction(PREFIX.to_string()))
    };

    match response {
        Ok(r) => Ok(interaction.create_response(&ctx.http, r).await?),
        Err(error) => {
            if let Error::RoleFull(_) = error {
                move_to_reserve(interaction, ctx).await?;
                Ok(())
            } else { Err(error) }
        }
    }?;
    Ok(())
}

async fn move_to_reserve(interaction: &ComponentInteraction, ctx: &Context) -> Result<()> {
    let mut data = EventKind::try_from(*interaction.message.clone()).unwrap();
    data.add_reserve(interaction.user.id);
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(data.get_embed())
    )).await?;
    interaction.create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
        .ephemeral(true)
        .embed(CreateEmbed::new().description("Rol lleno, se te ha movido a reserva!"))
    ).await?;
    Ok(())
}

fn get_selected_class(interaction: &ComponentInteraction) -> Option<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        Some(values.first().unwrap().to_string())
    } else { None }
}