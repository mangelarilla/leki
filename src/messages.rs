pub(crate) mod events;
mod help;

use crate::prelude::*;
use serenity::all::{ComponentInteraction, CreateInteractionResponse, ModalInteraction, Context, CommandInteraction};

#[shuttle_runtime::async_trait]
pub trait BotInteractionMessage {
    async fn modal(&self, interaction: &ModalInteraction, _ctx: &Context, _store: &Store) -> Result<CreateInteractionResponse> {
        Err(Error::UnknownInteraction(format!("{}", interaction.data.custom_id)))
    }
    async fn component(&self, interaction: &ComponentInteraction, _ctx: &Context, _store: &Store) -> Result<CreateInteractionResponse> {
        Err(Error::UnknownInteraction(format!("{}", interaction.data.custom_id)))
    }
    async fn command(&self, interaction: &CommandInteraction, _ctx: &Context, _store: &Store) -> Result<CreateInteractionResponse> {
        Err(Error::UnknownInteraction(format!("{}", interaction.data.name)))
    }
}