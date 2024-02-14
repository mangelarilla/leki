use std::time::Duration;
use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateModal, EditScheduledEvent, MessageId, ModalInteraction, Timestamp};
use crate::events::Event;
use crate::prelude::*;

pub(super) async fn edit_datetime(interaction: &ComponentInteraction, ctx: &Context, store: &Store, event: &Event, msg_id: MessageId) -> Result<ModalInteraction> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(CreateModal::new("edit_datetime_modal", "Cambiar Fecha y Hora")
        .components(vec![
            components::short_input("Fecha y hora (formato discord):", "edit_datetime_field", "<t:1707929820:F>", true)
        ]))).await?;
    if let Some(modal) = interaction.message.await_modal_interaction(&ctx.shard).await {
        let datetime = get_input_value(&modal.data.components, 0)
            .map(|dt| dt.replace("<t:", "").replace(":F>", ""))
            .map(|dt| dt.parse::<i64>().ok()).flatten()
            .map(|dt| DateTime::<Utc>::from_timestamp(dt, 0)).flatten();

        if let Some(datetime) = datetime {
            store.update_datetime(msg_id, datetime).await?;
            if let Some(event_id) = &event.scheduled_event {
                let guild = interaction.guild_id.unwrap();
                let duration: Duration = event.duration.into();
                let end_datetime = datetime + duration;
                guild.edit_scheduled_event(&ctx.http, event_id, EditScheduledEvent::new()
                    .start_time(Timestamp::from_unix_timestamp(datetime.timestamp())?)
                    .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp())?)
                ).await?;
            }
        }
        Ok(modal)
    } else {
        Err(Error::Timeout)
    }
}

pub(super) async fn edit_info(interaction: &ComponentInteraction, ctx: &Context, store: &Store, event: &Event, msg_id: MessageId) -> Result<ModalInteraction> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(CreateModal::new("edit_info_modal", "Cambiar info")
        .components(vec![
            components::short_input("Titulo", "edit_info_title", &event.title, false),
            components::short_input("Duracion", "edit_info_duration", &event.duration.to_string(), false),
            components::short_input("Descripcion", "edit_info_description", &event.description, false),
        ]))).await?;

    if let Some(modal) = interaction.message.await_modal_interaction(&ctx.shard).await {
        if let Some(title) = none_if_empty(get_input_value(&modal.data.components, 0)) {
            store.update_title(msg_id, title).await?;
        }
        if let Some(duration) = none_if_empty(get_input_value(&modal.data.components, 1)).map(|d| d.parse::<DurationString>().ok()).flatten() {
            store.update_duration(msg_id, duration).await?;
        }
        if let Some(description) = none_if_empty(get_input_value(&modal.data.components, 2)) {
            store.update_description(msg_id, description).await?;
        }
        Ok(modal)
    } else {
        Err(Error::Timeout)
    }
}

fn none_if_empty(src: Option<String>) -> Option<String> {
    src.map(|s| if s.is_empty() { None } else { Some(s) }).flatten()
}