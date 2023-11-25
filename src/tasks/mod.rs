use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use serenity::client::Context;
use serenity::model::id::UserId;
use tokio::task::JoinHandle;
use crate::prelude::*;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<UserId, JoinHandle<()>>> = Mutex::new(HashMap::new());
}

pub fn set_reminders(data: &TrialData, ctx: Arc<Context>) {
    let date = data.datetime.unwrap();
    for (_, player) in &data.dds {
        set_reminder(date.clone(), ctx.clone(), *player);
    }
    for (_, player) in &data.tanks {
        set_reminder(date.clone(), ctx.clone(), *player);
    }
    for (_, player) in &data.healers {
        set_reminder(date.clone(), ctx.clone(), *player);
    }
}

pub fn set_reminder(date: DateTime<Utc>, ctx: Arc<Context>, user: UserId) {
    unset_reminder(&user);
    let handle = tokio::spawn(async move {
        let duration = date - chrono::offset::Utc::now() - Duration::minutes(30);
        tracing::info!("{} minutes left", duration.num_minutes());
        if duration.num_minutes() > 0 {
            tokio::time::sleep(duration.to_std().unwrap()).await;
            let dm = user.create_dm_channel(&ctx.http).await.unwrap();
            dm.send_message(&ctx.http, |m| m.content("30 minutos para iniciar el evento!"))
                .await.unwrap();
        }
    });

    HASHMAP.lock().unwrap().insert(user, handle);
}

pub fn unset_reminder(user: &UserId) {
    let task = HASHMAP.lock().unwrap().remove(user);
    if let Some(task) = task {
        task.abort();
    }
}