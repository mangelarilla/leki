use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use serenity::all::{ChannelId, CreateMessage, Mention, MessageId, Ready};
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use shuttle_persist::PersistInstance;
use tokio::task::JoinHandle;
use tracing::{event, Instrument, instrument, Level, trace_span};
use crate::events::{Event, EventRole, Player};
use crate::prelude::*;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ChannelId, JoinHandle<()>>> = Mutex::new(HashMap::new());
}

pub(crate) async fn reset_all_reminders(ctx: Arc<Context>, ready: &Ready, store: Arc<PersistInstance>) {
    let span = trace_span!("ready_reminders");
    async move {
        for guild in &ready.guilds {
            let events = guild.id.scheduled_events(&ctx.http, false).await.unwrap();
            for event in events {
                if event.creator_id.unwrap() == ready.user.id {
                    let (_, channel_id, message) = parse_event_link(&event.description.unwrap());
                    let channel_id = ChannelId::new(channel_id);

                    let event = store.load::<Event>(message.to_string().as_str()).unwrap();

                    set_reminder(event.datetime.unwrap(), ctx.clone(), channel_id, MessageId::new(message), store.clone());
                }
            }
        }
    }.instrument(span).await
}

#[instrument]
pub fn set_reminder(date: DateTime<Utc>, ctx: Arc<Context>, channel: ChannelId, message: MessageId, store: Arc<PersistInstance>) {
    unset_reminder(&channel);
    let handle = tokio::spawn(async move {
        let duration = date - chrono::offset::Utc::now() - Duration::minutes(30);
        event!(Level::TRACE, "{} minutes left", duration.num_minutes());
        if duration.num_minutes() > 0 {
            tokio::time::sleep(duration.to_std().unwrap()).await;
            if let Some(event) = store.load::<Event>(message.to_string().as_str()).ok() {
                let signed_members: Vec<Player> = event.roles
                    .into_iter()
                    .filter_map(|(role, (players, _))| if role != EventRole::Reserve && role != EventRole::Absent {
                        Some(players)
                    } else { None })
                    .flatten()
                    .collect();

                channel.send_message(&ctx.http, CreateMessage::new()
                    .content(format!("__**Invitaciones para el RL**__\n```/script {}```", signed_members.iter()
                        .map(|u| {
                            format!("GroupInviteByName(\"@{}\")", u.name)
                        }).collect::<Vec<String>>().join(" ")
                    ))
                    .embed(CreateEmbed::new()
                        .title("⏰ 30 minutos para iniciar el evento!")
                        .field("Titulares", signed_members.iter()
                            .map(|u| {
                                Mention::User(u.id).to_string()
                            }).collect::<Vec<String>>()
                            .join("\n"), true)
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