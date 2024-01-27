use std::collections::HashMap;
use log::info;
use crate::messages::{BotInteractionFromCommandMessage, BotInteractionFromCommandMessageAsync, BotInteractionFromComponentMessage, BotInteractionFromComponentMessageAsync, BotInteractionMessage, BotInteractionModalMessage, BotMessageKind};


pub(crate) struct InteractionPipeline {
    messages: HashMap<String, BotMessageKind>,
}

impl InteractionPipeline {
    pub(crate) fn new() -> Self {
        InteractionPipeline {
            messages: HashMap::new()
        }
    }

    pub(crate) fn interaction(&mut self, event_id: impl Into<String>, handler: impl BotInteractionMessage + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), BotMessageKind::Interaction(Box::new(handler)));
        self
    }

    pub(crate) fn modal(&mut self, event_id: impl Into<String>, handler: impl BotInteractionModalMessage + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), BotMessageKind::FromModal(Box::new(handler)));
        self
    }

    pub(crate) fn message(&mut self, event_id: impl Into<String>, handler: impl BotInteractionFromComponentMessage + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), BotMessageKind::FromMessage(Box::new(handler)));
        self
    }

    pub(crate) fn message_async(&mut self, event_id: impl Into<String>, handler: impl BotInteractionFromComponentMessageAsync + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), BotMessageKind::FromMessageAsync(Box::new(handler)));
        self
    }

    pub(crate) fn command_async(&mut self, event_id: impl Into<String>, handler: impl BotInteractionFromCommandMessageAsync + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), BotMessageKind::FromCommandAsync(Box::new(handler)));
        self
    }

    pub(crate) fn command(&mut self, event_id: impl Into<String>, handler: impl BotInteractionFromCommandMessage + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), BotMessageKind::FromCommand(Box::new(handler)));
        self
    }

    pub(crate) fn get(&self, id: &str) -> Option<&BotMessageKind> {
        info!("Received interaction: {id}");
        if let Some((id, _)) = id.split_once("__") {
            self.messages.get(id)
        } else {
            self.messages.get(id)
        }
    }
}