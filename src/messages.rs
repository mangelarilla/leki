pub(crate) mod events;
mod help;

use async_trait::async_trait;
use crate::prelude::*;
use serenity::all::{ComponentInteraction, CreateInteractionResponse, ModalInteraction, Context, CommandInteraction};

pub enum BotMessageKind {
    Interaction(Box<dyn BotInteractionMessage>),
    FromModal(Box<dyn BotInteractionModalMessage>),
    FromMessage(Box<dyn BotInteractionFromComponentMessage>),
    FromMessageAsync(Box<dyn BotInteractionFromComponentMessageAsync>),
    FromCommandAsync(Box<dyn BotInteractionFromCommandMessageAsync>),
    FromCommand(Box<dyn BotInteractionFromCommandMessage>),
}

pub trait BotInteractionMessage: Send {
    fn message(&self) -> Result<CreateInteractionResponse>;
}

pub trait BotInteractionModalMessage: Send {
    fn message(&self, interaction: &ModalInteraction) -> Result<CreateInteractionResponse>;
}

pub trait BotInteractionFromComponentMessage: Send {
    fn message(&self, interaction: &ComponentInteraction) -> Result<CreateInteractionResponse>;
}

#[async_trait]
pub trait BotInteractionFromComponentMessageAsync: Sync + Send {
    async fn message(&self, interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse>;
}

#[async_trait]
pub trait BotInteractionFromCommandMessageAsync: Sync + Send {
    async fn message(&self, interaction: &CommandInteraction, ctx: &Context) -> Result<CreateInteractionResponse>;
}

pub trait BotInteractionFromCommandMessage: Send {
    fn message(&self, interaction: &CommandInteraction) -> Result<CreateInteractionResponse>;
}