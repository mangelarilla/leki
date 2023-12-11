use chrono::{Weekday};
use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, InputTextStyle};
use serenity::client::Context;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
        let response = CreateModal::new("event_dates", "Horas del evento")
            .components(values.into_iter().map(|day| {
                let localized = to_weekday_localized(&day.parse::<Weekday>().unwrap());
                CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, &localized, &localized)
                    .placeholder("18:00")
                    .required(true))
            }).collect());
        interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(response)).await?;
    }
    Ok(())
}