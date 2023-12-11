use serenity::all::{ButtonStyle, CommandInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateEmbed;
use serenity::client::Context;

pub(crate) async fn handle(ctx: &Context, command: CommandInteraction) {
    command.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().title("Gestion de eventos"))
            .ephemeral(true)
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("create_event")
                    .label("Crear evento")
                    .style(ButtonStyle::Success),
                CreateButton::new("update_event")
                    .label("Modificar evento")
                    .style(ButtonStyle::Primary),
                CreateButton::new("delete_event")
                    .label("Borrar evento")
                    .style(ButtonStyle::Danger),
            ])])
    )).await.unwrap();
}