use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

impl Display for InteractionPipeline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kv: Vec<String> = self.messages.keys()
            .map(|k| format!("{k}"))
            .collect();

        write!(f, "\n{}", kv.join("\n"))
    }
}