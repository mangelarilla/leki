mod sets;
mod entities;
mod error;
mod prelude;

use serenity::all::{CommandInteraction, CommandOptionType, CommandType, Context, CreateAutocompleteResponse, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildId, ReactionType};
use serenity::builder::{CreateCommand, CreateEmbed};
use strum::EnumProperty;
use prelude::*;
use crate::entities::GearQuality;
use crate::sets::armor::armour_set_request;
use crate::sets::{GearSet, SetEmbed};

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
            .embed(CreateEmbed::new().for_set(&gear_set))
            .select_menu(sets::quality_options())
    )).await?;

    let response = command.get_response(&ctx.http).await?;

    let interaction = response.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;
    let quality = get_selected_gear::<GearQuality>(&interaction).pop().unwrap();
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        sets::request_menu()
            .embed(CreateEmbed::new()
                .for_set(&gear_set)
                .with_quality(&quality)
            )
    )).await?;

    let interaction = response.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;
    if interaction.data.custom_id == "crafting_armour_set" {
        armour_set_request(&response, interaction, gear_set, quality, ctx).await?;
    }

    Ok(())
}

pub async fn register_commands(guild: GuildId, ctx: &Context) {
    guild.create_command(&ctx.http, CreateCommand::new("gear")
        .name_localized("es-ES", "equipo")
        .kind(CommandType::ChatInput)
        .description("Gear request")
        .description_localized("es-ES","Peticion de equipo")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "set", "Name of the gear set")
            .description_localized("es-ES", "Nombre del set de equipo, ejemplo: Colera de la Orden")
            .set_autocomplete(true)
            .required(true))
    ).await.unwrap();
}

