use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::{ChannelId, CreateMessage, GuildId, Mention, MessageId, UserId};
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use tokio::task::JoinHandle;
use tracing::{event, Instrument, instrument, Level, trace_span};
use crate::events::{EventRole, Player};
use crate::prelude::*;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ChannelId, JoinHandle<()>>> = Mutex::new(HashMap::new());
}

pub(crate) async fn reset_all_reminders(ctx: Arc<Context>, guild: GuildId, store: Arc<Store>) {
    let span = trace_span!("ready_reminders");
    async move {
        let events = guild.scheduled_events(&ctx.http, false).await.unwrap();
        for event in events {
            if event.creator_id.unwrap() == UserId::new(1148032756899643412) {
                let (_, channel_id, message) = parse_event_link(&event.description.unwrap());
                let channel_id = ChannelId::new(channel_id);

                let message = MessageId::new(message);
                let event = store.get_event(message).await.unwrap();

                set_reminder(event.datetime.unwrap(), ctx.clone(), channel_id, message, store.clone());
            }
        }
    }.instrument(span).await
}

#[instrument]
pub fn set_reminder(date: DateTime<Utc>, ctx: Arc<Context>, channel: ChannelId, message: MessageId, store: Arc<Store>) {
    unset_reminder(&channel);
    let handle = tokio::spawn(async move {
        let duration = date - chrono::offset::Utc::now() - Duration::try_minutes(30).unwrap();
        event!(Level::TRACE, "{} minutes left", duration.num_minutes());
        if duration.num_minutes() > 0 {
            tokio::time::sleep(duration.to_std().unwrap()).await;
            if let Some(event) = store.get_event(message).await.ok() {
                let signed_members: Vec<Player> = event.roles
                    .into_iter()
                    .filter_map(|pr| if pr.role != EventRole::Reserve && pr.role != EventRole::Absent {
                        Some(pr.players)
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

fn unset_reminder(channel: &ChannelId) {
    let task = HASHMAP.lock().unwrap().remove(channel);
    if let Some(task) = task {
        task.abort();
    }
}

#[instrument]
pub fn parse_event_link(text: &str) -> (u64, u64, u64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^https:\/\/discord\.com\/channels\/(?P<guild>\d+)\/(?P<channel>\d+)\/(?P<msg>\d+)$").unwrap();
    }

    RE.captures(text.lines().next().unwrap()).and_then(|cap| Option::from({
        (cap.name("guild").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap(),
         cap.name("channel").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap(),
         cap.name("msg").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap()
        )
    })).unwrap()
}