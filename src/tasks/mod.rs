use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use serenity::all::{ChannelId, CreateMessage, Mention, MessageId};
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use tokio::task::JoinHandle;
use crate::events::models::{EventSignups};
use crate::events::parse::ParseEventData;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ChannelId, JoinHandle<()>>> = Mutex::new(HashMap::new());
}

pub fn set_reminder(date: DateTime<Utc>, ctx: Arc<Context>, channel: ChannelId, message: MessageId) {
    unset_reminder(&channel);
    let handle = tokio::spawn(async move {
        let duration = date - chrono::offset::Utc::now() - Duration::minutes(30);
        tracing::info!("{} minutes left", duration.num_minutes());
        if duration.num_minutes() > 0 {
            tokio::time::sleep(duration.to_std().unwrap()).await;
            let message = channel.message(&ctx.http, message).await.unwrap();
            let event = message.parse_event().unwrap();
            let mentions: Vec<String> = event.signups().into_iter().map(|s| Mention::User(s).to_string()).collect();
            channel.send_message(&ctx.http, CreateMessage::new()
                .embed(CreateEmbed::new()
                    .title("⏰ 30 minutos para iniciar el evento!")
                    .field("Apuntados", mentions.join("\n"), true)
                )
            ).await.unwrap();
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