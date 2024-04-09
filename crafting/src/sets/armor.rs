use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse, Message, ReactionType};
use serenity::builder::{CreateEmbed, CreateSelectMenu};
use serenity::futures::StreamExt;
use strum::{EnumProperty, IntoEnumIterator};
use crate::entities::armour::{ArmourParts};
use crate::entities::GearQuality;
use crate::entities::traits::GearTraits;
use crate::prelude::*;
use crate::sets::GearPiece;

pub async fn armour_set_request(message: &Message, ctx: &Context) -> Result<Vec<GearPiece<ArmourParts>>> {

    let interaction = message.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;

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
        let (quality_interaction, selected_quality) = select_quality(&message, &trait_interaction, ctx, &selected_part).await?;
        interaction = quality_interaction;

        parts.push(GearPiece {part: selected_part.clone(), gear_trait: selected_trait, quality: selected_quality});
    }

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new()
                .title("Peticion de Set")
                .description("Adept Rider / Jinete Adepto")
                .field("Armadura", parts.iter().map(|p| p.to_string()).collect::<Vec<String>>().join("\n"), false)
            )
    )).await?;

    Ok(parts)
}

async fn select_trait(message: &Message, interaction: &ComponentInteraction, ctx: &Context, part: &ArmourParts) -> Result<(ComponentInteraction, GearTraits)> {
    let armour_trait = CreateSelectMenuKind::String {options: enum_to_options::<GearTraits>()};

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("armour_trait", armour_trait)
                    .placeholder(format!("Selecciona rasgo para {}", part.to_string()))
            )
    )).await?;

    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        let selected = get_selected_gear::<GearTraits>(&interaction).pop().unwrap();
        Ok((interaction, selected))
    } else {
        Err(Error::Timeout)
    }
}

async fn select_quality(message: &Message, interaction: &ComponentInteraction, ctx: &Context, part: &ArmourParts) -> Result<(ComponentInteraction, GearQuality)> {
    let armour_quality = CreateSelectMenuKind::String {
        options: GearQuality::iter()
            .map(|opt| CreateSelectMenuOption::new(opt.to_string(), opt.to_string())
                .emoji(ReactionType::Unicode(opt.get_str("Emoji").unwrap().to_string())))
            .collect()
    };

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("armour_quality", armour_quality)
                    .placeholder(format!("Selecciona calidad para {}", part.to_string()))
            )
    )).await?;

    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        let selected = get_selected_gear::<GearQuality>(&interaction).pop().unwrap();
        Ok((interaction, selected))
    } else {
        Err(Error::Timeout)
    }
}