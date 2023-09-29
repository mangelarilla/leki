use duration_string::DurationString;
use serenity::builder::{CreateEmbed};
use serenity::client::Context;
use serenity::model::id::{UserId};
use serenity::model::mention::Mention;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::modal::ModalSubmitInteraction;
use serenity::utils::Colour;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ModalSubmitInteraction) -> Result<()> {
    let title = get_text(&interaction.data.components, 0);
    let duration = get_text(&interaction.data.components, 1).parse::<DurationString>().map_err(anyhow::Error::msg)?;
    let description = get_text(&interaction.data.components, 2);
    let addons = get_text(&interaction.data.components, 3);
    let guides = get_text(&interaction.data.components, 4);

    interaction.create_interaction_response(&ctx.http,|r| r
        .kind(InteractionResponseType::UpdateMessage)
        .interaction_response_data(|m| m
            .set_embed(preview_embed(&title, &description, duration, &interaction.user.id, &addons, &guides))
            .components(|c| c.create_action_row(|r|
                r.create_select_menu(|s| s
                    .custom_id("event_days")
                    .max_values(5)
                    .placeholder("Dias de la semana")
                    .options(|opts| opts
                        .create_option(|opt| opt.label("Lunes").value("Monday"))
                        .create_option(|opt| opt.label("Martes").value("Tuesday"))
                        .create_option(|opt| opt.label("Miercoles").value("Wednesday"))
                        .create_option(|opt| opt.label("Jueves").value("Thursday"))
                        .create_option(|opt| opt.label("Viernes").value("Friday"))
                        .create_option(|opt| opt.label("Sabado").value("Saturday"))
                        .create_option(|opt| opt.label("Domingo").value("Sunday"))
                    )
                )
            ))
        )
    ).await?;

    // interaction.channel_id.send_message(&ctx.http, |m| m
    //     .set_embed(event_embed(&title, &description, &timestamp, duration, &interaction.user.id, &addons, &guides))
    //     .set_components(event_components())
    // ).await?;

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
    let mut embed = CreateEmbed::default();
    embed.author(|a| a.name("Previsualizacion"));
    embed.title(title);
    embed.description(description);
    embed.field(":date: Fecha y Hora:", "", true);
    embed.field(":hourglass_flowing_sand: Duración", duration, true);
    embed.field(":crown: Lider", Mention::User(*leader), true);
    embed.field("Guias:", addons, false);
    embed.field("AddOns recomendados:", guides, false);
    embed.field("", "\u{200b}", false);
    embed.field("", "\u{200b}", false);
    embed.field("<:tank:1154134006036713622> Tanks (0/2)", "", false);
    embed.field("<:dd:1154134731756150974> DD (0/8)", "", false);
    embed.field("<:healer:1154134924153065544> Healers (0/2)", "", false);
    embed.field(":wave: Reservas (0)", "", false);
    embed.field(":x: Ausencias (0)", "", false);
    embed.field("", "\u{200b}", false);
    embed.thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png");
    embed.color(Colour::from_rgb(0, 255, 0));
    embed
}