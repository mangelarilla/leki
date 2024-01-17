mod interactions;
mod error;
mod prelude;
mod utils;
mod tasks;
pub mod events;
mod commands;

use std::sync::Arc;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::model::prelude::{Interaction};
use serenity::prelude::*;
use tracing::{info};
use shuttle_secrets::SecretStore;
use crate::commands::register_commands;
use crate::tasks::reset_all_reminders;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        register_commands(&ctx.http, GuildId::new(1134046249293717514)).await;
        register_commands(&ctx.http, GuildId::new(592035476538392612)).await;

        reset_all_reminders(Arc::new(ctx), &ready).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                info!("Received command interaction: {}", &command.data.name);
                interactions::handle_commands(&ctx, command).await;
            }
            Interaction::Component(component) => {
                info!("Received component interaction: {}", &component.data.custom_id);
                interactions::handle_component(&ctx, component).await;
            }
            Interaction::Modal(modal) => {
                info!("Received component interaction: {}", &modal.data.custom_id);
                interactions::handle_modal(&ctx, modal).await;
            }
            _ => {}
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
