use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use serenity::all::{ChannelId, CreateMessage, GuildId, Mention, Message, MessageId, Ready};
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use tokio::task::JoinHandle;
use tracing::{event, Instrument, instrument, Level, trace_span};
use crate::events::generic::models::EventGenericData;
use crate::events::{EventData};
use crate::events::pvp::models::PvPData;
use crate::events::trials::models::TrialData;
use crate::prelude::*;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ChannelId, JoinHandle<()>>> = Mutex::new(HashMap::new());
}

pub(crate) async fn reset_all_reminders(ctx: Arc<Context>, ready: &Ready) {
    let span = trace_span!("ready_reminders");
    async move {
        for guild in &ready.guilds {
            let events = guild.id.scheduled_events(&ctx.http, false).await.unwrap();
            for event in events {
                if event.creator_id.unwrap() == ready.user.id {
                    let (guild, channel_id, message) = parse_event_link(&event.description.unwrap());
                    let guild_id = GuildId::new(guild);
                    let channel_id = ChannelId::new(channel_id);

                    let channel = guild_id.channels(&ctx.http).await.unwrap();
                    let message = channel.get(&channel_id).unwrap()
                        .message(&ctx.http, MessageId::new(message)).await.unwrap();

                    check_event_kinds(&message, ctx.clone(), channel_id, guild_id);
                }
            }
        }
    }.instrument(span).await
}

fn check_event_kinds(msg: &Message, ctx: Arc<Context>, channel_id: ChannelId, guild_id: GuildId) {
    if let Ok(trial) = TrialData::try_from(msg.clone()) {
        set_reminder::<TrialData>(trial.datetime().unwrap(), ctx, channel_id, msg.id, guild_id);
    }
    else if let Ok(pvp) = PvPData::try_from(msg.clone()) {
        set_reminder::<PvPData>(pvp.datetime().unwrap(), ctx, channel_id, msg.id, guild_id);
    }
    else if let Ok(generic) = EventGenericData::try_from(msg.clone()) {
        set_reminder::<EventGenericData>(generic.datetime().unwrap(), ctx, channel_id, msg.id, guild_id);
    }
}

#[instrument]
pub fn set_reminder<T: EventData + Send>(date: DateTime<Utc>, ctx: Arc<Context>, channel: ChannelId, message: MessageId, guild: GuildId) {
    unset_reminder(&channel);
    let handle = tokio::spawn(async move {
        let duration = date - chrono::offset::Utc::now() - Duration::minutes(30);
        event!(Level::TRACE, "{} minutes left", duration.num_minutes());
        if duration.num_minutes() > 0 {
            tokio::time::sleep(duration.to_std().unwrap()).await;
            let message = channel.message(&ctx.http, message).await.unwrap();
            if let Some(event) = T::try_from(message).ok() {
                let mentions: Vec<String> = event.signups().into_iter().map(|s| Mention::User(s.into()).to_string()).collect();
                let signed_members = event.signups().into_iter().map(|user| guild.member(&ctx.http, user));
                let signed_members = serenity::futures::future::join_all(signed_members).await;
                channel.send_message(&ctx.http, CreateMessage::new()
                    .content(format!("__**Invitaciones para el RL**__\n```/script {}```", signed_members.into_iter()
                        .map(|u| {
                            let member = u.unwrap();
                            format!("GroupInviteByName(\"@{}\")", member.nick.unwrap_or(member.user.name))
                        }).collect::<Vec<String>>().join(" ")
                    ))
                    .embed(CreateEmbed::new()
                        .title("⏰ 30 minutos para iniciar el evento!")
                        .field("Apuntados", mentions.join("\n"), true)
                    )
                ).await.unwrap();
            }
        }
    });

    HASHMAP.lock().unwrap().insert(channel, handle);
}

pub fn unset_reminder(channel: &ChannelId) {
    let task = HASHMAP.lock().unwrap().remove(channel);
    if let Some(task) = task {
        task.abort();
    }
}