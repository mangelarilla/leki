use duration_string::DurationString;
use serenity::all::{Colour, CreateActionRow, CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, ModalInteraction};
use serenity::builder::{CreateEmbed};
use serenity::client::Context;
use serenity::model::id::{UserId};
use serenity::model::mention::Mention;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
    let title = get_text(&interaction.data.components, 0);
    let duration = get_text(&interaction.data.components, 1).parse::<DurationString>()
        .map_err(|e| Error::DurationParse(anyhow::Error::msg(e)))?;
    let description = get_text(&interaction.data.components, 2);
    let addons = get_text(&interaction.data.components, 3);
    let guides = get_text(&interaction.data.components, 4);

    interaction.create_response(&ctx.http,CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(preview_embed(&title, &description, duration, &interaction.user.id, &addons, &guides))
            .components(vec![CreateActionRow::SelectMenu(
                CreateSelectMenu::new("event_days", CreateSelectMenuKind::String {
                    options: vec![
                        CreateSelectMenuOption::new("Lunes", "Monday"),
                        CreateSelectMenuOption::new("Martes", "Tuesday"),
                        CreateSelectMenuOption::new("Miercoles", "Wednesday"),
                        CreateSelectMenuOption::new("Jueves", "Thursday"),
                        CreateSelectMenuOption::new("Viernes", "Friday"),
                        CreateSelectMenuOption::new("Sabado", "Saturday"),
                        CreateSelectMenuOption::new("Domingo", "Sunday")
                    ]
                })
                    .max_values(5)
                    .placeholder("Dias de la semana")
            )])
    )).await?;

    Ok(())
}

fn preview_embed(
    title: &str,
    description: &str,
    duration: DurationString,
    leader: &UserId,
    addons: &str,
    guides: &str
) -> CreateEmbed {
    CreateEmbed::new()
        .author(CreateEmbedAuthor::new("Previsualizacion"))
        .title(title)
        .description(description)
        .field(":date: Fecha y Hora:", "", true)
        .field(":hourglass_flowing_sand: Duración", duration, true)
        .field(":crown: Lider", Mention::User(*leader).to_string(), true)
        .field("Guias:", addons, false)
        .field("AddOns recomendados:", guides, false)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field("<:tank:1154134006036713622> Tanks (0/2)", "", false)
        .field("<:dd:1154134731756150974> DD (0/8)", "", false)
        .field("<:healer:1154134924153065544> Healers (0/2)", "", false)
        .field(":wave: Reservas (0)", "", false)
        .field(":x: Ausencias (0)", "", false)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
        .color(Colour::from_rgb(0, 255, 0))
}