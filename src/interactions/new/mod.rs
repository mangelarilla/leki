mod trials;
mod pvp;
mod generic;

use std::fmt::Debug;
use std::sync::Arc;
use serenity::all::{ActionRowComponent, CommandInteraction, ComponentInteraction, ComponentInteractionDataKind, Context, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateMessage, CreateModal, CreateSelectMenu, Mention, Message, ModalInteraction, UserId};
use serenity::builder::CreateInteractionResponseMessage;
use crate::events::components::{event_comp_defaults_components, event_components_backup, event_scope_components, new_event_components, time_options};
use crate::events::embeds::new_event_embed;
use crate::events::models::{EventBasicData, EventComp, EventEmbed, EventKind, EventRole, FromBasicModal, FromComp, Player};
use crate::events::signup::EventSignupRoles;
use crate::interactions::{create_discord_event};
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
        Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
            .components(events::components::select_time(id, &channels))
        ))
    } else {
        unreachable!("The data kind is always a channel select")
    }
}

async fn create_event(interaction: &ComponentInteraction, ctx: &Context, is_pvp: bool) -> Result<CreateInteractionResponse> {
    let (channel, next_date) = events::get_date_time(&interaction.data).unwrap();
    let guild = interaction.guild_id.unwrap();
    let message = interaction.message.clone();

    let mut data = EventKind::try_from(*message.clone()).unwrap();
    data.set_datetime(next_date.clone());

    let msg = channel.send_message(&ctx.http, CreateMessage::new()
        .embed(data.get_embed())
        .components(if data.title().starts_with("[Roster Cerrado]") {
            vec![event_components_backup(if is_pvp {"signup_pvp_reserve"} else {"signup_trial_reserve"})]
        } else { data.get_components() })
    ).await.unwrap();
    create_discord_event(guild, ctx, &data, next_date, channel, msg.id, is_pvp).await?;
    tasks::set_reminder(data.datetime().unwrap(), Arc::new(ctx.clone()), channel, msg.id, guild);

    let remaining_times = message.components.iter()
        .filter_map(|r| {
            if let ActionRowComponent::SelectMenu(select) = r.components.first().unwrap() {
                let id = select.custom_id.clone().unwrap();
                if id != interaction.data.custom_id {
                    Some(CreateActionRow::SelectMenu(CreateSelectMenu::new(id, time_options())))
                } else { None }
            } else { None }
        })
        .collect();

    Ok(CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .add_embed(CreateEmbed::new().title("Nuevo evento!").description(format!("Evento creado en {}", Mention::Channel(channel).to_string())))
            .components(remaining_times)
    ))
}

fn request_event_comp_and_create_preview<T: FromBasicModal + EventEmbed + EventComp>(
    interaction: &ModalInteraction, id_confirm: impl Into<String>, id_change: impl Into<String>) -> CreateInteractionResponse {
    let event = T::from_basic_modal(&interaction.data.components, interaction.user.id);
    let response = CreateInteractionResponseMessage::new()
        .add_embed(event.get_embed_preview())
        .add_embed(T::get_comp_defaults_embed())
        .components(event_comp_defaults_components(id_confirm, id_change));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_event_scope(interaction: &ComponentInteraction, id_public: impl Into<String>, id_semi_public: impl Into<String>, id_private: impl Into<String>) -> CreateInteractionResponse {
    let event = EventKind::try_from(*interaction.message.clone()).unwrap();
    let response = CreateInteractionResponseMessage::new()
        .embed(event.get_embed_preview())
        .components(event_scope_components(id_public, id_semi_public, id_private));

    CreateInteractionResponse::UpdateMessage(response)
}

fn update_preview_and_request_event_scope<T: FromComp + EventEmbed>(interaction: &ModalInteraction, id_public: impl Into<String>, id_semi_public: impl Into<String>, id_private: impl Into<String>) -> CreateInteractionResponse {
    let event = T::from_comp_with_preview(&interaction.data.components, *interaction.message.clone().unwrap());
    let response = CreateInteractionResponseMessage::new()
        .embed(event.get_embed_preview())
        .components(event_scope_components(id_public, id_semi_public, id_private));

    CreateInteractionResponse::UpdateMessage(response)
}

fn request_new_comp<T: EventComp>(id: impl Into<String>) -> CreateInteractionResponse {
    let response = CreateModal::new(id, "Nueva Composicion")
        .components(T::get_comp_new_components());

    CreateInteractionResponse::Modal(response)
}

fn get_selected_users(interaction: &ComponentInteraction) -> Option<Vec<UserId>> {
    if let ComponentInteractionDataKind::UserSelect {values} = &interaction.data.kind {
        Some(values.clone())
    } else { None }
}

fn update_preview_with_role<T: TryFrom<Message> + EventEmbed + EventSignupRoles + Debug>(interaction: &ComponentInteraction, role: EventRole) -> CreateInteractionResponse {
    let selected_users = get_selected_users(interaction);
    let response = if let Some(users) = selected_users {
        if let Ok(mut event) = T::try_from(*interaction.message.clone()) {
            for user in users {
                event.signup(role, Player::Basic(user));
            }
            CreateInteractionResponseMessage::new()
                .embed(event.get_embed_preview())
        } else { CreateInteractionResponseMessage::new() }
    } else {
        CreateInteractionResponseMessage::new()
    };

    CreateInteractionResponse::UpdateMessage(response)
}