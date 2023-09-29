use serenity::model::prelude::{InteractionResponseType};
use serenity::model::prelude::component::{InputTextStyle};
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::Context;

use crate::prelude::*;
pub(crate) async fn handle(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(InteractionResponseType::Modal)
        .interaction_response_data(|d| d
            .custom_id("trial_texts")
            .title("Información de la trial")
            .components(|c| c
                .create_action_row(|r| r
                    .create_input_text(|i| i
                        .custom_id("trial_title")
                        .placeholder("Trial nivel avanzado - vDSR")
                        .label("Titulo de la trial")
                        .style(InputTextStyle::Short)
                        .required(true)
                    )
                )
                .create_action_row(|r| r
                    .create_input_text(|i| i
                        .custom_id("trial_duration")
                        .placeholder("2h")
                        .label("Duracion")
                        .style(InputTextStyle::Short)
                        .required(true)
                    )
                )
                .create_action_row(|r| r
                    .create_input_text(|i| i
                        .custom_id("trial_description")
                        .placeholder("Se empezara a montar 10 minutos antes\nbla bla bla")
                        .label("Descripción")
                        .style(InputTextStyle::Paragraph)
                        .required(false)
                    )
                )
                .create_action_row(|r| r
                    .create_input_text(|i| i
                        .custom_id("trial_addons")
                        .placeholder("[RaidNotifier](https://esoui.com/RaidNotifier)\n[CodeCombat](https://esoui.com/CodeCombat)")
                        .label("AddOns")
                        .style(InputTextStyle::Paragraph)
                        .required(false)
                    )
                )
                .create_action_row(|r| r
                    .create_input_text(|i| i
                        .custom_id("trial_guides")
                        .placeholder("[Alcast](https://alcast.com)\n[Xynode](https://xynode.com)")
                        .label("Guias")
                        .style(InputTextStyle::Paragraph)
                        .required(false)
                    )
                )
            )
        )
    ).await.unwrap();

    Ok(())
}