mod trials;
mod pvp;
mod generic;

use std::sync::Arc;
use serenity::all::{CommandInteraction, ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed, CreateInteractionResponse, CreateMessage, ModalInteraction, UserId};
use serenity::builder::CreateInteractionResponseMessage;
use crate::events::components::{event_components_backup, new_event_components};
use crate::events::embeds::new_event_embed;
use crate::events::models::{EventBasicData, EventKind};
use crate::interactions::{create_discord_event, get_event_channel_messages};
use crate::prelude::*;
use crate::tasks;

const PREFIX: &'static str = "new_";

pub(super) async fn new_event_response(interaction: &CommandInteraction, ctx: &Context) -> Result<()> {
    let response = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .add_embed(new_event_embed())
        .components(new_event_components(
            &format!("{}trial_event", PREFIX),
            &format!("{}pvp_event", PREFIX),
            &format!("{}generic_event", PREFIX)
        ));

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(response)).await?;
    Ok(())
}

pub(super) async fn handle_component(interaction: &ComponentInteraction, ctx: &Context) -> Result<()> {
    let response = if interaction.data.custom_id.starts_with(&format!("{}trial", PREFIX)) {
        Ok(trials::handle_component(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}pvp", PREFIX)) {
        Ok(pvp::handle_component(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}generic", PREFIX)) {
        Ok(generic::handle_component(interaction, ctx).await?)
    } else {
        Err(Error::UnknownInteraction(PREFIX.to_string()))
    }?;

    interaction.create_response(&ctx.http, response).await?;
    Ok(())
}

pub(super) async fn handle_modal(interaction: &ModalInteraction, ctx: &Context) -> Result<()> {
    let response = if interaction.data.custom_id.starts_with(&format!("{}trial", PREFIX)) {
        Ok(trials::handle_modal(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}pvp", PREFIX)) {
        Ok(pvp::handle_modal(interaction, ctx).await?)
    } else if interaction.data.custom_id.starts_with(&format!("{}generic", PREFIX)) {
        Ok(generic::handle_modal(interaction, ctx).await?)
    } else {
        Err(Error::UnknownInteraction(PREFIX.to_string()))
    }?;

    interaction.create_response(&ctx.http, response).await?;
    Ok(())
}

async fn request_event_times(id: &str, ctx: &Context, interaction: &ComponentInteraction) -> Result<CreateInteractionResponse> {
    if let ComponentInteractionDataKind::ChannelSelect { values } = &interaction.data.kind {
        let mut channels = vec![];
        for channel_id in values {
            let name = channel_id.name(&ctx.http).await?;
            channels.push((channel_id.clone(), get_channel_weekday(&name).unwrap()));
        }
        Ok(CreateInteractionResponse::Modal(events::select_time(id, &channels)))
    } else {
        unreachable!("The data kind is always a channel select")
    }
}

async fn create_event(interaction: &ModalInteraction, ctx: &Context, is_pvp: bool) -> Result<CreateInteractionResponse> {
    let mut count = 0;
    for (channel, next_date) in events::get_date_times(&interaction.data.components) {
        let guild = interaction.guild_id.unwrap();
        let messages = get_event_channel_messages(channel, ctx).await?;
        if messages.len() == 0 {
            let message = interaction.message.clone().unwrap();
            let mut data = EventKind::try_from(*message).unwrap();
            data.set_datetime(next_date.clone());
            let msg = channel.send_message(&ctx.http, CreateMessage::new()
                .embed(data.get_embed())
                .components(if data.title().starts_with("[Roster Cerrado]") {
                    vec![event_components_backup()]
                } else { data.get_components() })
            ).await.unwrap();
            create_discord_event(guild, ctx, &data, next_date, channel, msg.id, is_pvp).await?;
            tasks::set_reminder(data.datetime().unwrap(), Arc::new(ctx.clone()), channel, msg.id, guild);
            count += 1;
        }
    }
    Ok(CreateInteractionResponse::Defer(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(CreateEmbed::new()
                .description(format!("{} eventos creados! {}", count, if count == 0 {"Revisa que no esten ocupados esos dias :("} else{""})))
            .components(vec![])
    ))
}

fn get_selected_users(interaction: &ComponentInteraction) -> Option<Vec<UserId>> {
    if let ComponentInteractionDataKind::UserSelect {values} = &interaction.data.kind {
        Some(values.clone())
    } else { None }
}