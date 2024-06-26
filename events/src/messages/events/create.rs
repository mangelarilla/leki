mod composition;
mod info;
mod kind;
mod scope;
mod date;
mod role;

use std::sync::Arc;
use serenity::all::{ChannelId, Colour, CommandInteraction, Context, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateScheduledEvent, ExecuteWebhook, GuildId, Mention, MessageId, ScheduledEventId, ScheduledEventType, Timestamp, Webhook};
use sqlx::PgPool;
use crate::events::{Event, EventKind, EventRole, EventScopes};
use crate::prelude::*;
use crate::tasks;

pub async fn create_event(interaction: &CommandInteraction, ctx: &Context, pool: PgPool, announcement_hook: &str) -> Result<()> {
    let store = Store::new(pool);

    // Choose new event kind
    let (interaction, kind, message) = kind::select_event_kind(interaction, ctx).await?;

    // Request basic info
    let (modal, mut event) = info::request_info_modal(&message, &interaction, ctx, kind).await?;

    // Event composition
    let interaction = composition::handle_composition(&message, &modal, ctx, &mut event).await?;

    // Event notification role
    let interaction = role::select_role(&message, &interaction, ctx, &mut event).await?;

    // Event scope
    let interaction = scope::handle_scope(&message, &interaction, ctx, &mut event).await?;

    // Event datetime
    let (interaction, event_channel) = date::select_date(&message, &interaction, ctx, &mut event).await?;

    // Create event
    let image = event.image().await?;
    let event_message = event_channel.send_message(&ctx.http, CreateMessage::new()
        .content(event.notification_role.map(|r| Mention::Role(r).to_string()).unwrap_or("".to_string()))
        .embed(event.embed().attachment(image.filename))
        .components(signup_buttons(&event))
    ).await?;

    event.scheduled_event = Some(create_discord_event(interaction.guild_id.unwrap(), ctx, &event, event_channel, event_message.id).await?);

    store.create_event(event_message.id, &event).await?;

    tasks::set_reminder(event.datetime.clone().unwrap(), Arc::new(ctx.clone()), event_channel, event_message.id, Arc::new(store.clone()));

    send_announcement(ctx, &event, event_channel, announcement_hook).await?;

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(CreateEmbed::new().title("Nuevo evento!").description(format!("Evento creado en {}", Mention::Channel(event_channel).to_string())))
            .components(vec![])
    )).await?;

    Ok(())
}

fn signup_buttons(event: &Event) -> Vec<CreateActionRow> {
    let mut components = vec![];

    if event.scope != EventScopes::Private {
        components.push(CreateActionRow::Buttons(event.kind.roles()
            .into_iter()
            .filter_map(|r| if !r.is_backup_role() {
                Some(r.to_button(format!("signup_{}", r.to_id()), r.to_string()))
            } else { None }).collect()));

        components.push(CreateActionRow::Buttons(event.kind.roles()
            .into_iter()
            .filter_map(|r| if r.is_backup_role() {
                Some(r.to_button(format!("signup_{}", r.to_id()), r.to_string()))
            } else { None }).collect()));
    } else {
        components.push(CreateActionRow::Buttons(vec![
            EventRole::Absent.to_button(format!("signup_{}", EventRole::Absent.to_id()), EventRole::Absent.to_string())
        ]))
    }



    components
}

async fn create_discord_event(guild: GuildId, ctx: &Context, data: &Event, channel: ChannelId, msg: MessageId) -> Result<ScheduledEventId> {
    let date = data.datetime.unwrap();
    let duration: std::time::Duration = data.duration.into();
    let end_datetime = date + duration;
    let event = guild.create_scheduled_event(&ctx.http, CreateScheduledEvent::new(ScheduledEventType::Voice, &data.title, Timestamp::from_unix_timestamp(date.timestamp()).unwrap())
        .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, channel, msg, data.description))
        .channel_id(if guild == 1134046249293717514 {1157232748604444683} else {
            if data.kind == EventKind::PvP {1144350647848812564} else {1144350408769286274}
        })
        .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
        .image(&data.image().await?)
    ).await?;
    Ok(event.id)
}

async fn send_announcement(ctx: &Context, event: &Event, channel: ChannelId, hook: &str) -> Result<()> {
    let event_announcement = CreateEmbed::new()
        .title("Nuevo evento!")
        .field("Titulo", &event.title, false)
        .field(":hourglass_flowing_sand: Cuando", format!("<t:{}:F>", event.datetime.unwrap().timestamp()), true)
        .field(":house: Donde", Mention::Channel(channel).to_string(), true)
        .field("", &event.description, false)
        .color(Colour::from_rgb(0, 255, 0))
        .thumbnail(match event.kind {
            EventKind::Trial => "https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png",
            EventKind::PvP => "https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png"
        });

    let builder = ExecuteWebhook::new()
        .embed(event_announcement);

    Webhook::from_url(&ctx.http, hook).await?
        .execute(&ctx.http, false, builder).await?;

    Ok(())
}