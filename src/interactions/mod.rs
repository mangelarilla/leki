mod update_event;

use chrono::{DateTime, Utc};
use serenity::all::{ChannelId, CommandInteraction, ComponentInteraction, ComponentInteractionDataKind, GuildId, Message, MessageId, MessageType, ModalInteraction, ScheduledEventType};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage, CreateScheduledEvent, EditMessage, GetMessages};
use serenity::client::Context;
use serenity::model::Timestamp;
use tracing::{error};
use crate::events::generic::{event_embed};
use crate::events::models::EventBasicData;
use crate::events::parse::ParseEventData;
use crate::events::signup::EventBackupRoles;
use crate::events::trials::{trial_class, trial_embed, TrialRole};
use crate::events::trials::models::TrialData;
use crate::prelude::*;

pub(crate) async fn handle_commands(ctx: &Context, interaction: CommandInteraction) {
    let result = match interaction.data.name.as_str() {
        "events" => interaction.create_response(&ctx.http, CreateInteractionResponse::Message(events::new())).await.map_err(|o| o.into()),
        "Edit event" => update_event::handle(ctx, &interaction).await,
        "Delete event" => todo!(),
        _ => {
            error!("Command interaction '{}' not handled", &interaction.data.name);
            interaction.create_response(&ctx.http, not_implemented_response()).await.unwrap();
            Ok(())
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:#?}", &interaction.data.name);
        interaction.create_response(&ctx.http, error_response(error_msg(why))).await.unwrap();
    }
}

pub(crate) async fn handle_component(ctx: &Context, interaction: ComponentInteraction) {
    let result = match interaction.data.custom_id.as_str() {
        "trial_days" => handle_event_days("trial_time", ctx, &interaction).await,
        "ev_generic_days" => handle_event_days("event_time", ctx, &interaction).await,
        "create_trial" => {
            interaction.create_response(
                &ctx.http,
                CreateInteractionResponse::Modal(events::trials::data("trial_texts"))
            ).await.unwrap();
            Ok(())
        },
        "create_generic" => interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(events::data("ev_generic_data"))).await.map_err(|o| o.into()),
        "signup_tank" => signup_trial(&interaction, ctx, TrialRole::Tank, "tank_class").await,
        "signup_dd" => signup_trial(&interaction, ctx, TrialRole::DD, "dd_class").await,
        "signup_healer" => signup_trial(&interaction, ctx, TrialRole::Healer, "healer_class").await,
        "signup_reserve" => {
            let mut data = interaction.message.parse_event().unwrap();
            data.add_reserve(interaction.user.id);
            interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(data.get_embed())
            )).await.unwrap();
            Ok(())
        },
        "signup_absent" => {
            let mut data = interaction.message.parse_event().unwrap();
            data.add_absent(interaction.user.id);
            interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(data.get_embed())
            )).await.unwrap();
            Ok(())
        },
        "signup_event" => {
            let mut data = interaction.message.parse_generic().unwrap();
            data.signup(interaction.user.id);
            interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(event_embed(&data))
            )).await.unwrap();
            Ok(())
        },
        "healer_class" => signup_trial_class(&interaction, ctx, TrialRole::Healer).await,
        "dd_class" => signup_trial_class(&interaction, ctx, TrialRole::DD).await,
        "tank_class" => signup_trial_class(&interaction, ctx, TrialRole::Tank).await,
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
        "trial_texts" => {
            let msg = events::trials::select_date("event_days", &interaction.data.components, &interaction.user.id).unwrap();
            interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(msg)).await.unwrap();
            Ok(())
        },
        "trial_time" => create_event(&interaction, ctx).await,
        "event_time" => create_event(&interaction, ctx).await,
        "ev_generic_data" => {
            interaction.create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(events::select_date("ev_generic_days", &interaction.data.components, &interaction.user.id).unwrap())).await.map_err(|o| o.into())
        },
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

async fn create_event(interaction: &ModalInteraction, ctx: &Context) -> Result<()> {
    let mut count = 0;
    for (channel, next_date) in events::get_date_times(&interaction.data.components) {
        let guild = interaction.guild_id.unwrap();
        let messages = get_event_channel_messages(channel, ctx).await?;
        if messages.len() == 0 {
            let message = interaction.message.clone().unwrap();
            let mut data = message.parse_event().unwrap();
            data.set_datetime(next_date.clone());
            let msg = channel.send_message(&ctx.http, CreateMessage::new()
                .embed(data.get_embed())
                .components(data.get_components())
            ).await.unwrap();
            create_discord_event(guild, ctx, &data, next_date, msg.id).await?;
            count += 1;
        }
    }
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().description(format!("{} eventos creados!", count)))
            .components(vec![])
            .ephemeral(true)
    )).await.unwrap();
    Ok(())
}

fn get_selected_class(interaction: &ComponentInteraction) -> Option<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        Some(values.first().unwrap().to_string())
    } else { None }
}

async fn signup_trial_class(interaction: &ComponentInteraction, ctx: &Context, role: TrialRole) -> Result<()> {
    let selected_class = get_selected_class(interaction);
    let reference = interaction.message.message_reference.clone().unwrap();
    let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;
    let mut trial = original_msg.parse_trial()?;
    trial.signup(role, interaction.user.id, selected_class.unwrap());
    original_msg.edit(&ctx.http, EditMessage::new().embed(trial_embed(&trial))).await?;
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().description("Ya estas dentro!"))
            .components(vec![])
            .ephemeral(true)
    )).await?;
    Ok(())
}

async fn signup_trial(interaction: &ComponentInteraction, ctx: &Context, role: TrialRole, class_selector: &str) -> Result<()> {
    let mut data = interaction.message.parse_trial()?;
    if data.is_role_full(role) {
        move_to_reserve(&mut data, interaction, ctx).await?;
    } else {
        let class_selector = trial_class(class_selector);
        interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(class_selector)).await?;
    }
    Ok(())
}

async fn move_to_reserve(data: &mut TrialData, interaction: &ComponentInteraction, ctx: &Context) -> Result<()> {
    data.add_reserve(interaction.user.id);
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(trial_embed(&data))
    )).await?;
    interaction.create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
        .ephemeral(true)
        .embed(CreateEmbed::new().description("Rol lleno, se te ha movido a reserva!"))
    ).await?;
    Ok(())
}

async fn create_discord_event(guild: GuildId, ctx: &Context, data: &impl EventBasicData, date: DateTime<Utc>, msg: MessageId) -> Result<()> {
    let duration: std::time::Duration = data.duration().into();
    let end_datetime = date + duration;
    guild.create_scheduled_event(&ctx.http, CreateScheduledEvent::new(ScheduledEventType::Voice, data.title(), Timestamp::from_unix_timestamp(date.timestamp()).unwrap())
        .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, 1157232748604444683, msg, data.description().unwrap_or("".to_string())))
        .channel_id(1157232748604444683)
        .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
    ).await?;
    Ok(())
}

async fn handle_event_days(id: &str, ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if let ComponentInteractionDataKind::ChannelSelect { values } = &interaction.data.kind {
        let mut channels = vec![];
        for channel_id in values {
            let name = channel_id.name(&ctx.http).await.unwrap();
            channels.push((channel_id.clone(), get_channel_weekday(&name).unwrap()));
        }
        let response = events::select_time(id, &channels);
        interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(response)).await?;
    }
    Ok(())
}

async fn get_event_channel_messages(channel: ChannelId, ctx: &Context) -> Result<Vec<Message>> {
    let messages = channel.messages(&ctx.http, GetMessages::new().limit(10)).await?
        .into_iter()
        .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
        .collect::<Vec<Message>>();
    Ok(messages)
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