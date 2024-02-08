use std::str::FromStr;
use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditMessage, Mention};
use shuttle_persist::PersistInstance;
use crate::events::{Event, EventKind, EventRole, Player, PlayerClass};
use crate::messages::BotInteractionMessage;
use crate::prelude::*;

#[derive(Clone)]
pub struct SignupEvent {
    role: EventRole,
    kind: EventKind
}

impl SignupEvent {
    pub fn new(role: EventRole, kind: EventKind) -> Self {
        SignupEvent { role, kind }
    }
    pub fn flex_id(&self) -> String {
        format!("{}_{}_flex", self.kind, self.role.to_id())
    }
    pub fn class_id(&self) -> String {
        format!("{}_{}_class", self.kind, self.role.to_id())
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for SignupEvent {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &PersistInstance) -> Result<CreateInteractionResponse> {
        if self.class_id() == interaction.data.custom_id {
            let selected_class = get_selected_option(interaction).unwrap();
            let flex_roles = interaction.message.embeds.first().map(|e| e
                .description.clone().unwrap()
                .split(",")
                .filter_map(|f| EventRole::from_str(f).ok())
                .collect()).unwrap_or(vec![]);

            let reference = interaction.message.message_reference.clone().unwrap().message_id.unwrap();
            let mut original_msg = interaction.channel_id.message(&ctx.http, reference).await?;

            let mut event = store.load::<Event>(reference.to_string().as_str())?;

            let dm = event.leader.create_dm_channel(&ctx.http).await?;

            let user = Mention::User(interaction.user.id).to_string();
            let channel = Mention::Channel(interaction.channel_id).to_string();
            let flex = flex_roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();
            dm.send_message(&ctx.http, CreateMessage::new()
                .content(format!("{user} se ha apuntado al evento en {channel} como {}, y flexible a: {}", self.role, flex.join(",")))
            ).await?;
            let mut player = Player::new(interaction.user.id, interaction.member.clone().unwrap().nick.unwrap());
            player.class = PlayerClass::from_str(&selected_class).ok();
            player.flex = flex_roles;
            event.signup(self.role, player);

            original_msg.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;


            Ok(CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(CreateEmbed::new().description("Ya estas dentro!"))
                    .components(vec![])
            ))
        } else if self.flex_id() == interaction.data.custom_id {
            let selected_flex = get_selected_options(interaction);

            let response = if selected_flex.is_empty() {
                CreateInteractionResponseMessage::new()
            } else {
                CreateInteractionResponseMessage::new()
                    .embed(CreateEmbed::new().title("Roles de reserva").description(selected_flex.join(",")))
            };

            Ok(CreateInteractionResponse::UpdateMessage(response))
        } else if self.role == EventRole::Absent {
            let mut event = store.load::<Event>(interaction.message.id.to_string().as_str())?;
            let nick = interaction.member.as_ref().unwrap().nick.as_ref().unwrap();
            event.signup(self.role, Player::new(interaction.user.id, nick.to_string()));
            store.save(interaction.message.id.to_string().as_str(), &event)?;
            Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new().embed(event.embed())))
        } else {
            Ok(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .components(vec![select_flex_roles(self.class_id(), self.kind.roles()), select_player_class(self.class_id())])
            ))
        }
    }
}

pub(crate) fn select_player_class(id: impl Into<String>) -> CreateActionRow {
    let class_selector = CreateSelectMenuKind::String {
        options: vec![
            class_option(PlayerClass::Arcanist),
            class_option(PlayerClass::Necromancer),
            class_option(PlayerClass::Warden),
            class_option(PlayerClass::DragonKnight),
            class_option(PlayerClass::Templar),
            class_option(PlayerClass::Sorcerer),
            class_option(PlayerClass::NightBlade),
        ]
    };

    CreateActionRow::SelectMenu(CreateSelectMenu::new(id, class_selector)
        .placeholder("Selecciona clase"))
}

fn class_option(player_class: PlayerClass) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(player_class.to_string(), player_class.to_string())
        .description(player_class.label_es())
        .emoji(player_class.emoji())
}

pub(crate) fn select_flex_roles(id: impl Into<String>, roles: Vec<EventRole>) -> CreateActionRow {
    let role_selector = CreateSelectMenuKind::String {
        options: roles.iter().map(|r|
            CreateSelectMenuOption::new(r.to_string(), r.to_string())
        ).collect()
    };

    CreateActionRow::SelectMenu(CreateSelectMenu::new(id, role_selector)
        .placeholder("(Opcional) Roles de reserva")
        .max_values(roles.len() as u8))
}