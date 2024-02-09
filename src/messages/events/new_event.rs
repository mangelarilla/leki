use serenity::all::{ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::messages::{BotInteractionMessage};
use crate::prelude::*;

pub(crate) struct NewEvent {
    trial_id: String,
    pvp_id: String
}

impl NewEvent {
    pub(crate) fn new(trial_id: impl Into<String>, pvp_id: impl Into<String>) -> Self {
        NewEvent {
            trial_id: trial_id.into(),
            pvp_id: pvp_id.into()
        }
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for NewEvent {
    async fn command(&self, _interaction: &CommandInteraction, _ctx: &Context, _store: &Store) -> Result<CreateInteractionResponse> {
        let response = CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(embeds::basic("Nuevo evento", "Elige tipo de evento"))
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new(&self.trial_id)
                    .label("Trial")
                    .style(ButtonStyle::Secondary),
                CreateButton::new(&self.pvp_id)
                    .label("PvP")
                    .style(ButtonStyle::Secondary)
            ])]);

        Ok(CreateInteractionResponse::Message(response))
    }
}