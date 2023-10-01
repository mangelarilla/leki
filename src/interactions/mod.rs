mod create_event;
mod create_trial;
mod trial_texts;
mod event_days;
mod event_dates;
mod signup;
mod signup_class;

use serenity::builder::{CreateInteractionResponse};
use serenity::client::Context;
use serenity::model::prelude::{InteractionResponseType};
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::model::prelude::modal::ModalSubmitInteraction;
use tracing::{error};
use crate::prelude::*;

pub(crate) async fn handle_component(ctx: &Context, interaction: MessageComponentInteraction) {
    let result = match interaction.data.custom_id.as_str() {
        "create_event" => create_event::handle(ctx, &interaction).await,
        "event_days" => event_days::handle(ctx, &interaction).await,
        "create_trial" => create_trial::handle(ctx, &interaction).await,
        "signup_tank" => signup::tank(ctx, &interaction).await,
        "signup_dd" => signup::dd(ctx, &interaction).await,
        "signup_healer" => signup::healer(ctx, &interaction).await,
        "healer_class" => signup_class::healer(ctx, &interaction).await,
        "dd_class" => signup_class::dd(ctx, &interaction).await,
        "tank_class" => signup_class::tank(ctx, &interaction).await,
        _ => {
            error!("Component interaction '{}' not handled", &interaction.data.custom_id);
            interaction.create_interaction_response(&ctx.http, not_implemented_response).await.unwrap();
            Ok(())
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:?}", &interaction.data.custom_id);
        interaction.create_interaction_response(&ctx.http, |r|
            error_response(r, error_msg(why))
        ).await.unwrap();
    }
}

pub(crate) async fn handle_modal(ctx: &Context, interaction: ModalSubmitInteraction) {
    let result = match interaction.data.custom_id.as_str() {
        "trial_texts" => trial_texts::handle(ctx, &interaction).await,
        "event_dates" => event_dates::handle(ctx, &interaction).await,
        _ => {
            error!("Component interaction '{}' not handled", &interaction.data.custom_id);
            interaction.create_interaction_response(&ctx.http, not_implemented_response).await.unwrap();
            Ok(())
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:?}", &interaction.data.custom_id);
        interaction.create_interaction_response(&ctx.http, |r|
            error_response(r, error_msg(why))).await.unwrap();
    }
}

fn error_msg(why: Error) -> &'static str {
    match why {
        Error::Timestamp(_) => "Te has inventado la fecha bro",
        Error::ParseInt(_) => "Te has inventado la hora bro",
        Error::DurationParse(_) => "Te has inventado la duracion, ejemplos validos: 1h, 2h30m",
        _ => "Wooops"
    }
}

fn not_implemented_response<'a, 'b>(interaction: &'a mut CreateInteractionResponse<'b>) -> &'a mut CreateInteractionResponse<'b> {
    error_response(interaction, "Estamos trabajando en ello :D")
}

fn error_response<'a, 'b>(interaction: &'a mut CreateInteractionResponse<'b>, msg: &str) -> &'a mut CreateInteractionResponse<'b> {
    interaction
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|d| d
            .embed(|e| e
                .description(msg)
            )
            .ephemeral(true)
        )
}