mod interactions;
mod error;
mod prelude;
mod slash_commands;
mod utils;
mod tasks;

use std::sync::Arc;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, MessageId};
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Interaction};
use serenity::prelude::*;
use tracing::{info};
use shuttle_secrets::SecretStore;
use crate::utils::{parse_event_link, parse_trial_data};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        let command = Command::create_global_application_command(&ctx.http, |command|
            command
                .name("events")
                .description("Event management")
                .description_localized("es-ES", "Gestión de eventos")
        ).await;

        if let Ok(command) = command {
            info!("Command {} registered", &command.name);
        }

        let ctx = Arc::new(ctx);
        for guild in &ready.guilds {
                let events = guild.id.scheduled_events(&ctx.http, false).await.unwrap();
                for event in events {
                    if event.creator_id.unwrap() == ready.user.id {
                        let (guild, channel_id, message) = parse_event_link(&event.description.unwrap());
                        let channel = GuildId(guild).channels(&ctx.http).await.unwrap();
                        let message = channel.get(&ChannelId(channel_id)).unwrap()
                            .message(&ctx.http, MessageId(message)).await.unwrap();
                        let data = parse_trial_data(&message).unwrap();
                        tasks::set_reminders(&data, ctx.clone());
                    }
                }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Ping(_) => {}
            Interaction::ApplicationCommand(command) => {
                info!("Received command interaction: {}", &command.data.name);
                slash_commands::events::handle(&ctx, command).await;
            }
            Interaction::MessageComponent(component) => {
                info!("Received component interaction: {}", &component.data.custom_id);
                interactions::handle_component(&ctx, component).await;
            }
            Interaction::Autocomplete(_) => {}
            Interaction::ModalSubmit(modal) => {
                info!("Received component interaction: {}", &modal.data.custom_id);
                interactions::handle_modal(&ctx, modal).await;
            }
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
