use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenuKind, CreateSelectMenuOption, Message};
use serenity::builder::{CreateSelectMenu};
use crate::entities::jewelry::Jewelries;
use crate::entities::traits::jewelry_traits;
use crate::prelude::*;
use crate::sets::{GearPiece};

pub async fn jewelry_set_request(message: &Message, interaction: ComponentInteraction, ctx: &Context) -> Result<(ComponentInteraction, Vec<GearPiece<Jewelries>>)> {
    let jewelry_parts = CreateSelectMenuKind::String {options: vec![
        CreateSelectMenuOption::new(Jewelries::Necklace.to_string(), Jewelries::Necklace.to_string()),
        CreateSelectMenuOption::new(Jewelries::Ring.to_string(), format!("{}_1", Jewelries::Ring.to_string())),
        CreateSelectMenuOption::new(Jewelries::Ring.to_string(), format!("{}_2", Jewelries::Ring.to_string()))
    ]};

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("crafting_jewelries", jewelry_parts)
                    .placeholder("Selecciona toda la joyeria que quieres pedir")
                    .max_values(3)
            )
    )).await?;

    let mut interaction = message.await_component_interaction(&ctx.shard).await.ok_or(Error::Timeout)?;
    let mut parts = vec![];
    for selected_part in get_selected_gear::<Jewelries>(&interaction) {
        let (trait_interaction, selected_trait) = super::select_trait(&message, &interaction, ctx, selected_part.to_string(), jewelry_traits()).await?;
        interaction = trait_interaction;

        parts.push(GearPiece {part: selected_part.clone(), gear_trait: selected_trait});
    }

    Ok((interaction, parts))
}