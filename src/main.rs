mod error;
mod prelude;
mod utils;
mod tasks;
pub mod events;
mod commands;
mod messages;
mod store;

use std::sync::Arc;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::model::prelude::{Interaction};
use serenity::prelude::*;
use tracing::{error, info};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use crate::commands::register_commands;
use crate::prelude::Store;
use crate::tasks::reset_all_reminders;

struct Bot {
    guild: GuildId,
    store: Store
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> shuttle_serenity::ShuttleSerenity {
    sqlx::migrate!().run(&pool).await.expect("Migrations failed :(");

    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let guild = if let Some(guild) = secret_store.get("DISCORD_GUILD") {
        match guild.parse::<u64>() {
            Ok(id) => GuildId::new(id),
            Err(e) => return Err(anyhow!("{}", e).into())
        }
    } else {
        return Err(anyhow!("'DISCORD_GUILD' was not found").into());
    };

    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { guild, store: Store::new(pool) })
        .await
        .expect("Err creating client");

    Ok(client.into())
}

fn get_interaction_guild(interaction: &Interaction) -> Option<GuildId> {
    match interaction {
        Interaction::Command(c) => c.guild_id,
        Interaction::Autocomplete(a) => a.guild_id,
        Interaction::Component(c) => c.guild_id,
        Interaction::Modal(m) => m.guild_id,
        _ => None
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        register_commands(&ctx, self.guild).await;

        reset_all_reminders(Arc::new(ctx), self.guild, Arc::new(self.store.clone())).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if get_interaction_guild(&interaction).is_some_and(|g| g != self.guild) {
            return;
        }

        match interaction {
            Interaction::Command(command) => {
                info!("Command interaction: {}", command.data.name);
                if command.data.name == "events" {
                    if let Err(why) = messages::events::create_event(&command, &ctx, &self.store).await {
                        error!("Create event: {why:#?}");
                    }
                }

                if command.data.name == "Edit event" {
                    if let Err(why) = messages::events::edit_event(&command, &ctx, &self.store).await {
                        error!("Edit event: {why:#?}");
                    }
                }

                if command.data.name == "Delete event" {
                    if let Err(why) = messages::events::delete_event(&command, &ctx, &self.store).await {
                        error!("Edit event: {why:#?}");
                    }
                }
            }
            Interaction::Component(component) => {
                info!("Component interaction: {}", component.data.custom_id);
                if component.data.custom_id.starts_with("signup") {
                    if let Err(why) = messages::events::signup_event(&component, &ctx, &self.store).await {
                        error!("Signup event: {why:#?}");
                    }
                }
            }
            Interaction::Modal(m) => {info!("Modal interaction: {}", m.data.custom_id)}
            _ => {}
        }
    }
}
