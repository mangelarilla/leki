use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, Message};
use serenity::builder::{CreateSelectMenu};
use crate::entities::traits::{weapon_traits};
use crate::entities::weapon::Weapons;
use crate::prelude::*;
use crate::sets::{GearPiece};

pub async fn weapon_set_request(message: &Message, interaction: ComponentInteraction, ctx: &Context) -> Result<(ComponentInteraction, Vec<GearPiece<Weapons>>)> {
    let armour_parts = CreateSelectMenuKind::String {options: Weapons::select_options()};
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("crafting_weapons", armour_parts)
                    .placeholder("Selecciona todas las armas que quieres pedir")
                    .max_values(12)
            )
    )).await?;

    let mut interaction = message.await_component_interaction(&ctx.shard).await.ok_or(Error::Timeout)?;
    let mut parts = vec![];
    for selected_part in get_selected_gear::<Weapons>(&interaction) {
        let (trait_interaction, selected_trait) = super::select_trait(&message, &interaction, ctx, selected_part.to_string(), weapon_traits()).await?;
        interaction = trait_interaction;

        parts.push(GearPiece {part: selected_part.clone(), gear_trait: selected_trait});
    }

    Ok((interaction, parts))
}