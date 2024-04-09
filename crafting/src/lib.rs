mod sets;
mod entities;
mod error;
mod prelude;

use serenity::all::{ButtonStyle, ChannelId, Color, CommandInteraction, CommandOptionType, CommandType, Context, CreateAutocompleteResponse, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildId, ReactionType};
use serenity::builder::{CreateButton, CreateCommand, CreateEmbed};
use tracing::log::info;
use prelude::*;
use crate::sets::armor::armour_set_request;
use crate::sets::GearSet;

pub async fn gear_set_autocomplete(command: CommandInteraction, ctx: &Context) -> serenity::Result<()> {
    let option = command.data.options.first().unwrap();
    let value = option.value.as_str().unwrap_or("");
    let choices = sets::gear_sets()
        .into_iter()
        .filter(|gs| gs.matches(value))
        .map(|gs| gs.to_autocomplete_choice())
        .take(10)
        .collect();

    let response = CreateInteractionResponse::Autocomplete(
        CreateAutocompleteResponse::new()
            .set_choices(choices)
    );

    command.create_response(&ctx.http, response).await
}

pub async fn gear_set_request(command: &CommandInteraction, ctx: &Context) -> Result<()> {
    let option = command.data.options.first().unwrap();
    let value = option.value.as_str().ok_or(Error::InvalidGearSet("None".to_string()))?;
    let gear_set = GearSet::try_from(value.to_string())?;

    command.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(CreateEmbed::new().title(format!("Set: {gear_set}")).description("Configura la peticion de equipo"))
            .button(CreateButton::new("crafting_armour_set").label("Armadura").emoji(ReactionType::Unicode("🛡️".to_string())))
            .button(CreateButton::new("crafting_weapon_set").label("Armas").emoji(ReactionType::Unicode("⚔️".to_string())))
            .button(CreateButton::new("crafting_jewelry_set").label("Joyeria").emoji(ReactionType::Unicode("💎".to_string())))
    )).await?;

    let response = command.get_response(&ctx.http).await?;

    let parts = armour_set_request(&response, ctx).await?;

    response.channel_id.send_message(&ctx, CreateMessage::new()
        .content("@Fabricantes")
        .embed(
            CreateEmbed::new()
                .title("🔨 Peticion de Set")
                .description(format!("{gear_set}"))
                .color(Color::from_rgb(0, 255, 0))
        )
        .button(CreateButton::new("request_en").label("Ver encargo").emoji(ReactionType::Unicode("🇬🇧".to_string())))
        .button(CreateButton::new("request_es").label("Ver encargo").emoji(ReactionType::Unicode("🇪🇸".to_string())))
        .button(CreateButton::new("request_accept").label("Aceptar encargo").style(ButtonStyle::Success))
    ).await?;

    response.channel_id.send_message(&ctx, CreateMessage::new()
        .content("Encargo aceptado por **polerokfi**")
        .embed(
            CreateEmbed::new()
                .title("🔨 Peticion de Set")
                .description(format!("{gear_set}"))
                .color(Color::from_rgb(255, 0, 0))
        )
        .button(CreateButton::new("request_en").label("Ver encargo").emoji(ReactionType::Unicode("🇬🇧".to_string())))
        .button(CreateButton::new("request_es").label("Ver encargo").emoji(ReactionType::Unicode("🇪🇸".to_string())))
    ).await?;

    Ok(())
}

pub async fn register_commands(guild: GuildId, ctx: &Context) {
    let command = guild.create_command(&ctx.http, CreateCommand::new("gear")
        .name_localized("es-ES", "equipo")
        .kind(CommandType::ChatInput)
        .description("Gear request")
        .description_localized("es-ES","Peticion de equipo")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "set", "Name of the gear set")
            .description_localized("es-ES", "Nombre del set de equipo, ejemplo: Colera de la Orden")
            .set_autocomplete(true)
            .required(true))
    ).await.unwrap();

    ChannelId::new(1134051372640247959)
        .send_message(&ctx.http, CreateMessage::new()
            .embed(CreateEmbed::new()
                .title("Como hacer una peticion de crafteo")
                .field("", format!("Para hacer una peticion de equipo usa </{}:{}>", command.name, command.id.get()), false)
            )).await.unwrap();
}

