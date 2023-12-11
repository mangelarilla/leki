mod create_event;
mod create_trial;
mod trial_texts;
mod event_days;
mod event_dates;
mod signup;
mod signup_class;
mod update_event;

use serenity::all::{ComponentInteraction, CreateInteractionResponseMessage, ModalInteraction};
use serenity::builder::{CreateInteractionResponse};
use serenity::client::Context;
use tracing::{error};
use crate::prelude::*;

pub(crate) async fn handle_component(ctx: &Context, interaction: ComponentInteraction) {
    let result = match interaction.data.custom_id.as_str() {
        "create_event" => create_event::handle(ctx, &interaction).await,
        "update_event" => update_event::handle(ctx, &interaction).await,
        "event_days" => event_days::handle(ctx, &interaction).await,
        "create_trial" => create_trial::handle(ctx, &interaction).await,
        "signup_tank" => signup::tank(ctx, &interaction).await,
        "signup_dd" => signup::dd(ctx, &interaction).await,
        "signup_healer" => signup::healer(ctx, &interaction).await,
        "signup_reserve" => signup::reserve(ctx, &interaction).await,
        "signup_absent" => signup::absent(ctx, &interaction).await,
        "healer_class" => signup_class::healer(ctx, &interaction).await,
        "dd_class" => signup_class::dd(ctx, &interaction).await,
        "tank_class" => signup_class::tank(ctx, &interaction).await,
        _ => {
            error!("Component interaction '{}' not handled", &interaction.data.custom_id);
            interaction.create_response(&ctx.http, not_implemented_response()).await.unwrap();
            Ok(())
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:#?}", &interaction.data.custom_id);
        interaction.create_response(&ctx.http, error_response(error_msg(why))).await.unwrap();
    }
}

pub(crate) async fn handle_modal(ctx: &Context, interaction: ModalInteraction) {
    let result = match interaction.data.custom_id.as_str() {
        "trial_texts" => trial_texts::handle(ctx, &interaction).await,
        "event_dates" => event_dates::handle(ctx, &interaction).await,
        _ => {
            error!("Component interaction '{}' not handled", &interaction.data.custom_id);
            interaction.create_response(&ctx.http, not_implemented_response()).await.unwrap();
            Ok(())
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:?}", &interaction.data.custom_id);
        interaction.create_response(&ctx.http, error_response(error_msg(why))).await.unwrap();
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

fn not_implemented_response() -> CreateInteractionResponse {
    error_response("Estamos trabajando en ello :D")
}

fn error_response(msg: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(msg)
            .ephemeral(true)
    )
}