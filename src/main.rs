mod interactions;
mod error;
mod prelude;
mod slash_commands;
mod utils;

use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Interaction};
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

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
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let client = Client::builder(&token, GatewayIntents::DIRECT_MESSAGES)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}