use chrono::{Weekday};
use serenity::client::Context;
use serenity::model::application::component::InputTextStyle;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(InteractionResponseType::Modal)
        .interaction_response_data(|d| d
            .custom_id("event_dates")
            .title("Horas del evento")
            .components(|c| {
                for day in &interaction.data.values {
                    let localized = match day.parse::<Weekday>().unwrap() {
                        Weekday::Mon => "lunes",
                        Weekday::Tue => "martes",
                        Weekday::Wed => "miercoles",
                        Weekday::Thu => "jueves",
                        Weekday::Fri => "viernes",
                        Weekday::Sat => "sabado",
                        Weekday::Sun => "domingo"
                    };
                    c.create_action_row(|r| r
                        .create_input_text(|i| i
                            .custom_id(localized)
                            .placeholder("18:00")
                            .label(localized)
                            .style(InputTextStyle::Short)
                            .required(true)
                        )
                    );
                }
                c
            })
        )
    ).await?;

    Ok(())
}