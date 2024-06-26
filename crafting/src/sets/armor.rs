use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, Message};
use serenity::builder::{CreateSelectMenu};
use crate::entities::armour::{ArmourParts};
use crate::entities::traits::armour_traits;
use crate::prelude::*;
use crate::sets::{GearPiece};

pub async fn armour_set_request(message: &Message, interaction: ComponentInteraction, ctx: &Context) -> Result<(ComponentInteraction, Vec<GearPiece<ArmourParts>>)> {
    let armour_parts = CreateSelectMenuKind::String {options: enum_to_options::<ArmourParts>()};
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("crafting_armour_parts", armour_parts)
                    .placeholder("Selecciona toda la armadura que quieres pedir")
                    .max_values(12)
            )
    )).await?;

    let mut interaction = message.await_component_interaction(&ctx.shard).await.ok_or(Error::Timeout)?;
    let mut parts = vec![];
    for selected_part in get_selected_gear::<ArmourParts>(&interaction) {
        let (trait_interaction, selected_trait) = super::select_trait(&message, &interaction, ctx, selected_part.to_string(), armour_traits()).await?;
        interaction = trait_interaction;

        parts.push(GearPiece {part: selected_part.clone(), gear_trait: selected_trait});
    }

    Ok((interaction, parts))
}