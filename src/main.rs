mod commands;

use std::sync::Arc;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::model::prelude::{Interaction};
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use sqlx::PgPool;
use crate::commands::register_commands;

struct Bot {
    guild: GuildId,
    pool: PgPool,
    announcement_hook: String
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> shuttle_serenity::ShuttleSerenity {
    sqlx::migrate!().run(&pool).await.expect("Migrations failed :(");

    let token = secret_store.get("DISCORD_TOKEN")
        .expect("'DISCORD_TOKEN' was not found");
    let guild = secret_store.get("DISCORD_GUILD")
        .map(|guild| guild.parse::<u64>().expect("DISCORD_GUILD invalid u64"))
        .map(|id| GuildId::new(id))
        .expect("'DISCORD_GUILD' was not found");
    let announcement_hook = secret_store.get("DISCORD_ANNOUNCEMENTS_HOOK")
        .expect("'DISCORD_TOKEN' was not found");

    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { guild, pool, announcement_hook })
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
        crafting::register_commands(self.guild, &ctx).await;

        events::tasks::reset_all_reminders(Arc::new(ctx), self.guild, self.pool.clone()).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if get_interaction_guild(&interaction).is_some_and(|g| g != self.guild) {
            return;
        }

        match interaction {
            Interaction::Command(command) => {
                info!("Command interaction: {}", command.data.name);
                if command.data.name == "events" {
                    if let Err(why) = events::messages::events::create_event(&command, &ctx, self.pool.clone(), &self.announcement_hook).await {
                        error!("Create event: {why:#?}");
                    }
                }

                if command.data.name == "Edit event" {
                    if let Err(why) = events::messages::events::edit_event(&command, &ctx, self.pool.clone()).await {
                        error!("Edit event: {why:#?}");
                    }
                }

                if command.data.name == "Delete event" {
                    if let Err(why) = events::messages::events::delete_event(&command, &ctx, self.pool.clone()).await {
                        error!("Edit event: {why:#?}");
                    }
                }

                if command.data.name == "gear" {
                    if let Err(why) = crafting::gear_set_request(&command, &ctx).await {
                        error!("Edit event: {why:#?}");
                    }
                }
            }
            Interaction::Component(component) => {
                info!("Component interaction: {}", component.data.custom_id);

                if component.data.custom_id.starts_with("signup") {
                    if let Err(why) = events::messages::events::signup_event(&component, &ctx, self.pool.clone()).await {
                        error!("Signup event: {why:#?}");
                    }
                }
            }
            Interaction::Modal(m) => {info!("Modal interaction: {}", m.data.custom_id)},
            Interaction::Autocomplete(command) => {
                crafting::gear_set_autocomplete(command, &ctx).await.unwrap();
            }
            _ => {}
        }
    }
}
