use serenity::all::{ChannelType, ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};
use shuttle_persist::PersistInstance;
use crate::events::{EventKind};
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::BotInteractionMessage;
use crate::messages::events::{CreateEvent};
use crate::prelude::get_selected_channels;

#[derive(Clone)]
pub(crate) struct SelectDate {
    day_id: String,
    time_id: String
}

impl SelectDate {
    pub(crate) fn new(kind: EventKind, pipeline: &mut InteractionPipeline) -> Self {
        let date = SelectDate { day_id: format!("{kind}_event_day"), time_id: format!("{kind}_event_time") };
        pipeline.add(&date.day_id, date.clone());
        let create = CreateEvent::new(kind, pipeline);
        pipeline.add(&date.time_id, create);

        date
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for SelectDate {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, _store: &PersistInstance) -> crate::prelude::Result<CreateInteractionResponse> {
        let components = if interaction.data.custom_id == self.day_id {
            let channel_selector = CreateSelectMenuKind::Channel {
                channel_types: Some(vec![ChannelType::Text]),
                default_channels: None,
            };

            vec![CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(&self.day_id, channel_selector)
                        .placeholder("Canales del evento"))]
        } else {
            let channels = get_selected_channels(interaction);
            let channel = channels.first().unwrap();
            let name = channel.name(&ctx.http).await?;
            let day = get_channel_weekday(&name).unwrap();

            let time_options = CreateSelectMenuKind::String {
                options: vec![
                    time_option("16:00"), time_option("16:30"),
                    time_option("17:00"), time_option("17:30"),
                    time_option("18:00"), time_option("18:30"),
                    time_option("19:00"), time_option("19:30"),
                    time_option("20:00"), time_option("20:30"),
                    time_option("21:00"), time_option("21:30"),
                    time_option("22:00"), time_option("22:30"),
                    time_option("23:00"), time_option("23:30"),
                ]
            };

            vec![CreateActionRow::SelectMenu(
                CreateSelectMenu::new(format!("{}__{}_{}", &self.time_id, channel, day), time_options)
                    .placeholder(format!("Selecciona hora para el {}", day))
            )]
        };


        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new().components(components)
        ))
    }
}

fn get_channel_weekday(channel_name: &str) -> Option<String> {
    let weekdays = vec!["lunes", "martes", "miercoles", "jueves", "viernes", "sabado", "domingo"];
    for weekday in weekdays {
        let channel_no_accents = unidecode::unidecode(channel_name);
        if channel_no_accents.contains(weekday) {
            return Some(weekday.to_string());
        }
    }

    None
}

fn time_option(time: &str) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(time, time)
}
