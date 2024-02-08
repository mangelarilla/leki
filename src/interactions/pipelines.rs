use std::collections::HashMap;
use tracing::info;
use crate::messages::BotInteractionMessage;


pub(crate) struct InteractionPipeline {
    messages: HashMap<String, Box<dyn BotInteractionMessage + Sync + Send>>,
}

impl InteractionPipeline {
    pub(crate) fn new() -> Self {
        InteractionPipeline {
            messages: HashMap::new()
        }
    }

    pub(crate) fn add(&mut self, event_id: impl Into<String>, handler: impl BotInteractionMessage + Sync + Send + 'static) -> &mut Self {
        self.messages.insert(event_id.into(), Box::new(handler));
        self
    }

    pub(crate) fn get(&self, id: &str) -> Option<&Box<dyn BotInteractionMessage + Sync + Send>> {
        info!("Received interaction: {id}");
        if let Some((id, _)) = id.split_once("__") {
            self.messages.get(id)
        } else {
            self.messages.get(id)
        }
    }
}