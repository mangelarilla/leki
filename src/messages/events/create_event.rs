use std::any::TypeId;
use std::marker::PhantomData;
use std::sync::Arc;
use async_trait::async_trait;
use serenity::all::{ActionRowComponent, ComponentInteraction, Context, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, Mention, Message};
use crate::{events, tasks};
use crate::error::Error;
use crate::events::components::{time_options};
use crate::events::EventData;
use crate::events::pvp::models::PvPData;
use crate::messages::BotInteractionFromComponentMessageAsync;

pub(crate) struct CreateEvent<T: EventData + 'static + Sync + Send> {
    phantom: PhantomData<T>
}

impl<T: EventData + 'static + Sync + Send> CreateEvent<T> {
    pub(crate) fn new() -> Self {
        CreateEvent { phantom: PhantomData }
    }
}

#[async_trait]
impl<T: EventData + 'static + Sync + Send> BotInteractionFromComponentMessageAsync for CreateEvent<T> where Error: From<<T as TryFrom<Message>>::Error> {
    async fn message(&self, interaction: &ComponentInteraction, ctx: &Context) -> crate::prelude::Result<CreateInteractionResponse> {
        let (channel, next_date) = events::get_date_time(&interaction.data).unwrap();
        let guild = interaction.guild_id.unwrap();
        let is_pvp = TypeId::of::<T>() == TypeId::of::<PvPData>();

        let mut data = T::try_from(*interaction.message.clone())?;
        data.set_datetime(next_date.clone());

        let components = if data.title().starts_with("[Roster Cerrado]") {
            vec![CreateActionRow::Buttons(T::backup_buttons())]
        } else {
            vec![CreateActionRow::Buttons(T::role_buttons()), CreateActionRow::Buttons(T::backup_buttons())]
        };

        let msg = channel.send_message(&ctx.http, CreateMessage::new()
            .embed(data.get_embed())
            .components(components)
        ).await?;
        crate::interactions::create_discord_event(guild, ctx, &data, next_date, channel, msg.id, is_pvp).await?;
        tasks::set_reminder::<T>(data.datetime().unwrap(), Arc::new(ctx.clone()), channel, msg.id, guild);

        let remaining_times = interaction.message.components.iter()
            .filter_map(|r| {
                if let ActionRowComponent::SelectMenu(select) = r.components.first().unwrap() {
                    let id = select.custom_id.clone().unwrap();
                    if id != interaction.data.custom_id {
                        Some(CreateActionRow::SelectMenu(CreateSelectMenu::new(id, time_options())))
                    } else { None }
                } else { None }
            })
            .collect();

        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .add_embed(CreateEmbed::new().title("Nuevo evento!").description(format!("Evento creado en {}", Mention::Channel(channel).to_string())))
                .components(remaining_times)
        ))
    }
}