use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, Message};
use serenity::builder::{CreateSelectMenu};
use serenity::futures::StreamExt;
use crate::entities::armour::{ArmourParts};
use crate::entities::GearQuality;
use crate::entities::traits::GearTraits;
use crate::prelude::*;
use crate::sets::{GearPiece, GearSet, SetEmbed};

pub async fn armour_set_request(message: &Message, interaction: ComponentInteraction, gear_set: GearSet, quality: GearQuality, ctx: &Context) -> Result<()> {
    let armour_parts = CreateSelectMenuKind::String {options: enum_to_options::<ArmourParts>()};
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("crafting_armour_parts", armour_parts)
                    .placeholder("Selecciona todas las partes que quieres pedir")
                    .max_values(12)
            )
    )).await?;

    let mut interaction = message.await_component_interaction(&ctx.shard).await.ok_or(Error::Timeout)?;
    let mut parts = vec![];
    for selected_part in get_selected_gear::<ArmourParts>(&interaction) {
        let (trait_interaction, selected_trait) = select_trait(&message, &interaction, ctx, &selected_part).await?;
        interaction = trait_interaction;

        parts.push(GearPiece {part: selected_part.clone(), gear_trait: selected_trait});
    }

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .button(CreateButton::new("crafting_confirmation").label("Confirmar").style(ButtonStyle::Success))
            .embed(CreateEmbed::new()
                .for_set(&gear_set)
                .with_quality(&quality)
                .with_armor(&parts)
            )
    )).await?;

    Ok(())
}

async fn select_trait(message: &Message, interaction: &ComponentInteraction, ctx: &Context, part: &ArmourParts) -> Result<(ComponentInteraction, GearTraits)> {
    let armour_trait = CreateSelectMenuKind::String {options: enum_to_options::<GearTraits>()};

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("armour_trait", armour_trait)
                    .placeholder(format!("Rasgo para {}", part.to_string()))
            )
    )).await?;

    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        let selected = get_selected_gear::<GearTraits>(&interaction).pop().unwrap();
        Ok((interaction, selected))
    } else {
        Err(Error::Timeout)
    }
}