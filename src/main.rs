mod interactions;
mod error;
mod prelude;
mod utils;
mod tasks;
pub mod events;

use std::sync::Arc;
use anyhow::anyhow;
use serenity::all::{Command, CommandType, CreateCommand};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, MessageId};
use serenity::model::prelude::{Interaction};
use serenity::prelude::*;
use tracing::{error, info};
use shuttle_secrets::SecretStore;
use crate::events::models::{EventBasicData, EventKind};
use crate::utils::{parse_event_link};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        register_command(&ctx.http, CreateCommand::new("events")
            .description("Event management")
            .description_localized("es-ES", "Gestión de eventos")
        ).await;
        register_command(&ctx.http, CreateCommand::new("Edit event")
            .name_localized("es-ES","Editar evento")
            .kind(CommandType::Message)
        ).await;
        register_command(&ctx.http, CreateCommand::new("Delete event")
            .name_localized("es-ES","Eliminar evento")
            .kind(CommandType::Message)
        ).await;
        register_command(&ctx.http, CreateCommand::new("help")
            .description("Como se usa Leki")
        ).await;

        let ctx = Arc::new(ctx);
        for guild in &ready.guilds {
                let events = guild.id.scheduled_events(&ctx.http, false).await.unwrap();
                for event in events {
                    if event.creator_id.unwrap() == ready.user.id {
                        let (guild, channel_id, message) = parse_event_link(&event.description.unwrap());
                        let channel = GuildId::new(guild).channels(&ctx.http).await.unwrap();
                        let message = channel.get(&ChannelId::new(channel_id)).unwrap()
                            .message(&ctx.http, MessageId::new(message)).await.unwrap();
                        let event = EventKind::try_from(message.clone()).unwrap();
                        tasks::set_reminder(event.datetime().unwrap(), ctx.clone(), ChannelId::new(channel_id), message.id, GuildId::new(guild));
                    }
                }
        }
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

async fn register_command(http: impl CacheHttp, builder: CreateCommand) {
    let command = Command::create_global_command(http, builder).await;

    match command {
        Ok(command) => info!("Command '{}' registered", &command.name),
        Err(error) => error!("Error registering command: {}",  error)
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
