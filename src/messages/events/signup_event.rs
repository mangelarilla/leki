use std::str::FromStr;
use serenity::all::{ChannelId, ComponentInteraction, Context, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditMessage, Member, Mention, RoleId};
use tracing::{info, instrument};
use crate::events::{EventRole, Player, PlayerClass};
use crate::messages::BotInteractionMessage;
use crate::prelude::*;

#[derive(Clone)]
pub struct SignupEvent {
    role: EventRole
}

impl SignupEvent {
    pub fn new(role: EventRole) -> Self {
        SignupEvent { role }
    }
    pub fn flex_id(&self) -> String {
        format!("{}_flex", self.role.to_id())
    }
    pub fn class_id(&self) -> String {
        format!("{}_class", self.role.to_id())
    }
}

#[instrument]
fn signup_msg(member: Member) -> CreateEmbed {
    // Role Escudero
    let tax = if member.roles.contains(&RoleId::new(592733654996746253)) {"3"} else {"10"};
    info!("Member {} signed in with roles: {:?}", member.display_name(), member.roles);
    CreateEmbed::new()
        .title("Ya estas dentro!")
        .description(format!("Recuerda que __si no eres reserva__ y faltas de manera __injustificada__ deberas ingresar {tax}k al banco como penalización tal y como indican las {}",
        Mention::Channel(ChannelId::new(1004447678689714197)).to_string())) // #normas
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for SignupEvent {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        if interaction.data.custom_id.ends_with(&self.class_id()) {
            let selected_class = get_selected_option(interaction).unwrap();
            let flex_roles = interaction.message.embeds.first().map(|e| e
                .description.clone().unwrap()
                .split(",")
                .filter_map(|f| EventRole::from_str(f).ok())
                .collect()).unwrap_or(vec![]);
            let flex_as_string = flex_roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();

            let reference = interaction.message.message_reference.clone().unwrap().message_id.unwrap();

            let mut player = Player::new(interaction.user.id, interaction.member.clone().unwrap().display_name().to_string());
            player.class = PlayerClass::from_str(&selected_class).ok();
            player.flex = flex_roles;

            let event = store.get_event(reference).await?;
            if let Some(pr) = event.roles.iter().find(|p| p.role == self.role) {
                if pr.max.is_some_and(|m| m <= pr.players.len()) {
                    player.flex.push(self.role);
                    store.signup_player(reference, EventRole::Reserve, player).await?;
                } else {
                    store.signup_player(reference, self.role, player).await?;
                }
            }

            let event = store.get_event(reference).await?;

            let dm = event.leader.create_dm_channel(&ctx.http).await?;
            let user = Mention::User(interaction.user.id).to_string();
            let channel = Mention::Channel(interaction.channel_id).to_string();

            dm.send_message(&ctx.http, CreateMessage::new()
                .content(format!("{user} se ha apuntado al evento en {channel} como {}, y flexible a: {}", self.role, flex_as_string.join(",")))
            ).await?;


            let mut original_msg = interaction.channel_id.message(&ctx.http, reference).await?;
            original_msg.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;


            Ok(CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(signup_msg(interaction.member.clone().unwrap()))
                    .components(vec![])
            ))
        } else if interaction.data.custom_id.ends_with(&self.flex_id()) {
            let selected_flex = get_selected_options(interaction);

            let response = if selected_flex.is_empty() {
                CreateInteractionResponseMessage::new()
            } else {
                CreateInteractionResponseMessage::new()
                    .embed(CreateEmbed::new().title("Roles de reserva").description(selected_flex.join(",")))
            };

            Ok(CreateInteractionResponse::UpdateMessage(response))
        } else if self.role == EventRole::Absent {
            let nick = interaction.member.as_ref().unwrap().nick.as_ref().unwrap();
            store.signup_player(interaction.message.id, self.role, Player::new(interaction.user.id, nick.to_string())).await?;
            let event = store.get_event(interaction.message.id).await?;
            Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new().embed(event.embed())))
        } else {
            let event = store.get_event(interaction.message.id).await?;
            Ok(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .components(vec![select_flex_roles(self.flex_id(), event.kind.roles()), select_player_class(self.class_id())])
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