use crate::prelude::*;
use serenity::model::prelude::*;
use serenity::model::prelude::component::*;
use serenity::model::prelude::message_component::*;
use serenity::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(InteractionResponseType::UpdateMessage)
        .interaction_response_data(|d| d
            .embed(|e| e
                .title("Nuevo evento")
                .description("Elige tipo de evento")
            )
            .components(|c| c.create_action_row(|r| r
                .create_button(|b| b
                    .label("Trial")
                    .custom_id("create_trial")
                    .style(ButtonStyle::Secondary)
                )
                .create_button(|b| b
                    .label("PvP")
                    .custom_id("create_pvp")
                    .style(ButtonStyle::Secondary)
                )
                .create_button(|b| b
                    .label("Generico")
                    .custom_id("create_generic")
                    .style(ButtonStyle::Secondary)
                )
            ))
        )
    ).await?;
    Ok(())
}