use serenity::all::{CreateActionRow, CreateInteractionResponseMessage, CreateSelectMenuKind};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateSelectMenu, CreateSelectMenuOption};
use crate::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {

    let events = interaction.guild_id.unwrap()
        .scheduled_events(&ctx.http, false).await?;

    let options = events.into_iter().filter_map(|event| {
        if event.creator_id.unwrap().get() == interaction.application_id.get() {
            let (_, channel_id, message) = parse_event_link(&event.description.unwrap());
            Some(CreateSelectMenuOption::new(
                format!("{} - {}", event.name, event.start_time.weekday()),
                format!("{channel_id}:{message}")
            ))
        } else { None }
    }).collect();

    let interaction_response_msg = CreateInteractionResponseMessage::new()
        .embed(CreateEmbed::new().title("Editar eventos").description("Elige tipo de evento"))
        .components(vec![CreateActionRow::SelectMenu(
            CreateSelectMenu::new("edit_event", CreateSelectMenuKind::String { options })
                .max_values(1)
                .placeholder("Selecciona evento")
        )]);

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(interaction_response_msg)).await?;
    Ok(())
}