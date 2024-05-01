mod sets;
mod entities;
mod error;
mod prelude;
mod store;

use serenity::all::{ButtonStyle, CommandInteraction, CommandOptionType, CommandType, Context, CreateAutocompleteResponse, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId};
use serenity::builder::{CreateButton, CreateCommand};
use prelude::*;
use crate::entities::GearQuality;
use crate::sets::armor::armour_set_request;
use crate::sets::{GearSet};
use crate::sets::jewelry::jewelry_set_request;
use crate::sets::request::GearRequest;
use crate::sets::weapons::weapon_set_request;

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
    let mut request = GearRequest::new(GearSet::try_from(value.to_string())?);

    command.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(request.to_embed_preview())
            .select_menu(sets::quality_options())
    )).await?;

    let response = command.get_response(&ctx.http).await?;

    let interaction = response.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;
    request.with_quality(get_selected_gear::<GearQuality>(&interaction).pop().unwrap());
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        sets::request_menu()
            .embed(request.to_embed_preview())
    )).await?;

    let mut interaction = response.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;

    while interaction.data.custom_id.as_str() != "crafting_set_confirmation" {
        match interaction.data.custom_id.as_str() {
            "crafting_armour_set" => {
                let (armor_interaction, parts) = armour_set_request(&response, interaction, ctx).await?;
                interaction = armor_interaction;
                request.set_armour(parts);
            }
            "crafting_jewelry_set" => {
                let (jewelry_interaction, parts) = jewelry_set_request(&response, interaction, ctx).await?;
                interaction = jewelry_interaction;
                request.set_jewelry(parts);
            }
            "crafting_weapon_set" => {
                let (wep_interaction, weapons) = weapon_set_request(&response, interaction, ctx).await?;
                interaction = wep_interaction;
                request.set_weapons(weapons);
            }
            _ => {}
        }

        interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
            sets::request_menu()
                .add_embed(request.to_embed_preview())
                .add_embed(request.to_embed_cost()?)
                .button(CreateButton::new("crafting_set_confirmation")
                    .label("Confirmar (pendiente)")
                    .style(ButtonStyle::Success))
        )).await?;

        interaction = response.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;
    }

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new().content("confirmado!")
    )).await?;
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

