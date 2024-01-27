use std::marker::PhantomData;
use std::str::FromStr;
use async_trait::async_trait;
use serenity::all::{ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, EditMessage, Mention};
use crate::events::{EventData, EventRole, EventSignedRole, Player};
use crate::messages::{BotInteractionFromComponentMessage, BotInteractionFromComponentMessageAsync};
use crate::prelude::*;

pub struct SignupEventClass<T: EventData + Sync + Send> {
    phantom: PhantomData<T>,
    role: EventRole
}

impl<T: EventData + Sync + Send> SignupEventClass<T> {
    pub fn new(role: EventRole) -> Self {
        SignupEventClass { role, phantom: PhantomData }
    }
}

impl<T: EventData + Sync + Send> BotInteractionFromComponentMessage for SignupEventClass<T> {
    fn message(&self, interaction: &ComponentInteraction) -> Result<CreateInteractionResponse> {
        let selected_flex = get_selected_options(interaction);

        let response = if selected_flex.is_empty() {
            CreateInteractionResponseMessage::new()
        } else {
            CreateInteractionResponseMessage::new()
                .embed(CreateEmbed::new().title("Roles de reserva").description(selected_flex.join(",")))
        };

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}

#[async_trait]
impl<T: EventData + Sync + Send> BotInteractionFromComponentMessageAsync for SignupEventClass<T>
    where Error: From<<T as TryFrom<serenity::all::Message>>::Error> {
    async fn message(&self, interaction: &ComponentInteraction, ctx: &Context) -> Result<CreateInteractionResponse> {
        let selected_class = get_selected_option(interaction).unwrap();
        let mut flex_roles = interaction.message.embeds.first().map(|e| e
            .description.clone().unwrap()
            .split(",")
            .filter_map(|f| EventSignedRole::from_str(f).ok())
            .collect()).unwrap_or(vec![]);

        let reference = interaction.message.message_reference.clone().unwrap();
        let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;

        let mut event = T::try_from(original_msg.clone())?;

        let dm = event.leader().create_dm_channel(&ctx.http).await?;
        match self.role {
            EventRole::Signed(role) => {
                if event.is_role_full(role) {
                    if !flex_roles.contains(&role) {
                        flex_roles.push(role);
                    }
                    event.add_reserve(Player::Class(interaction.user.id, selected_class, flex_roles))
                } else {
                    let user = Mention::User(interaction.user.id).to_string();
                    let channel = Mention::Channel(interaction.channel_id).to_string();
                    let flex = flex_roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();
                    dm.send_message(&ctx.http, CreateMessage::new()
                        .content(format!("{user} se ha apuntado al evento en {channel} como {}, y flexible a: {}", role, flex.join(",")))
                    ).await?;
                    event.signup(role, Player::Class(interaction.user.id, selected_class, flex_roles));
                }
            }
            EventRole::Reserve => event.add_reserve(Player::Class(interaction.user.id, selected_class, flex_roles)),
            EventRole::Absent => event.add_absent(interaction.user.id)
        }

        original_msg.edit(&ctx.http, EditMessage::new().embed(event.get_embed())).await?;


        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(CreateEmbed::new().description("Ya estas dentro!"))
                .components(vec![])
        ))
    }
}