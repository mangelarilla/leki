use std::str::FromStr;
use serenity::all::{ChannelId, ComponentInteraction, Context, CreateEmbed, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditMessage, Member, Mention, RoleId, UserId};
use serenity::builder::CreateInteractionResponse;
use crate::events::{EventKind, EventRole, Player, PlayerClass};
use crate::prelude::*;
use serenity::futures::StreamExt;
use tracing::{info, instrument};

pub async fn signup_event(interaction: &ComponentInteraction, ctx: &Context, store: &Store) -> Result<()> {
    if let Some(role) = EventRole::from_partial_id(&interaction.data.custom_id) {
        let mut original_message = interaction.message.clone();
        let mut event = store.get_event(interaction.message.id).await?;
        let member = interaction.member.clone().unwrap();
        let mut player = Player::new(interaction.user.id, member.display_name());

        let dm = event.leader.create_dm_channel(&ctx.http).await?;
        let user = Mention::User(interaction.user.id).to_string();
        let channel = Mention::Channel(interaction.channel_id).to_string();

        if role == EventRole::Absent {
            store.signup_player(original_message.id, EventRole::Absent, &player).await?;
            event.add_player(EventRole::Absent, player);
            original_message.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(signup_msg(&member, None, event.leader))).await?;

            dm.send_message(&ctx.http, CreateMessage::new()
                .content(format!("{user} no va a poder asistir al evento en {channel}"))
            ).await?;
        } else {

            // Select flex roles and class
            interaction.create_response(&ctx.http, select_class_flex(event.kind)).await?;
            let mut selects = interaction.get_response(&ctx.http).await?
                .await_component_interaction(&ctx.shard)
                .stream();

            while let Some(interaction) = selects.next().await {
                if interaction.data.custom_id.ends_with("flex") {
                    let selected_flex = get_selected_options(&interaction);
                    interaction.create_response(&ctx.http, update_flex_roles(selected_flex)).await?;
                } else {
                    player.class = PlayerClass::from_str(&get_selected_option(&interaction).unwrap()).ok();
                    player.flex = interaction.message.embeds.first().map(|e| e
                        .description.clone().unwrap()
                        .split(",")
                        .filter_map(|f| EventRole::from_str(f).ok())
                        .collect()).unwrap_or(vec![]);
                    let flex_as_string = player.flex.iter().map(|r| r.to_string()).collect::<Vec<String>>();

                    if event.notification_role.is_some_and(|r| !member.roles.contains(&r)) {
                        player.flex.push(role);
                        event.add_player(EventRole::Reserve, player.clone());
                        store.signup_player(original_message.id, EventRole::Reserve, &player).await?;

                        dm.send_message(&ctx.http, CreateMessage::new()
                            .content(format!("{user} no cumple los requisitos de titular y se ha movido a reserva en el evento de {channel}, flexible a: {}", flex_as_string.join(",")))
                        ).await?;
                    } else {
                        let role = event.add_player(role, player.clone());
                        store.signup_player(original_message.id, role, &player).await?;

                        dm.send_message(&ctx.http, CreateMessage::new()
                            .content(format!("{user} se ha apuntado al evento en {channel} como {role}, y flexible a: {}", flex_as_string.join(",")))
                        ).await?;
                    }

                    original_message.edit(&ctx.http, EditMessage::new().embed(event.embed())).await?;
                    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(signup_msg(&member, event.notification_role, event.leader))).await?;
                }
            }
        }
    }

    Ok(())
}

fn select_class_flex(kind: EventKind) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .select_menu(select_flex_roles(kind.roles()))
            .select_menu(select_player_class())
    )
}

fn select_player_class() -> CreateSelectMenu {
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

    CreateSelectMenu::new("signup_class", class_selector)
        .placeholder("Selecciona clase")
}

fn class_option(player_class: PlayerClass) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(player_class.to_string(), player_class.to_string())
        .description(player_class.label_es())
        .emoji(player_class.emoji())
}

fn select_flex_roles(roles: Vec<EventRole>) -> CreateSelectMenu {
    let role_selector = CreateSelectMenuKind::String {
        options: roles.iter().map(|r|
            CreateSelectMenuOption::new(r.to_string(), r.to_string())
        ).collect()
    };

    CreateSelectMenu::new("signup_flex", role_selector)
        .placeholder("(Opcional) Roles de reserva")
        .max_values(roles.len() as u8)
}

fn update_flex_roles(flex_roles: Vec<String>) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().title("Roles de reserva").description(flex_roles.join(",")))
    )
}

#[instrument]
fn signup_msg(member: &Member, notification_role: Option<RoleId>, leader: UserId) -> CreateInteractionResponseMessage {
    // Role Escudero
    let tax = if member.roles.contains(&RoleId::new(592733654996746253)) {"3"} else {"10"};
    info!("Member {} signed in with roles: {:?}", member.display_name(), member.roles);

    let rules_channel = Mention::Channel(ChannelId::new(1004447678689714197)).to_string();
    let frac = format!("Recuerda que __si no eres reserva__ y faltas de manera __injustificada__ deberas ingresar {tax}k al banco como penalización tal y como indican las {rules_channel}");
    let embed = if notification_role.is_some_and(|r| !member.roles.contains(&r)) {
        let notification_role = Mention::Role(notification_role.unwrap()).to_string();
        CreateEmbed::new()
            .title("Apuntado como reserva")
            .description(format!(r#"
Para apuntarte como titular deberas formar parte de {notification_role}, consulta los requisitos de rosters de la norma **1.5** en {rules_channel}
Si crees que cumples los requisitos o quieres mas informacion consultar con el lider del evento {}

{frac}"#, Mention::User(leader)))
    } else {
        CreateEmbed::new()
            .title("Ya estas dentro!")
            .description(frac)
    };

    CreateInteractionResponseMessage::new()
        .content(Mention::User(member.user.id).to_string())
        .ephemeral(true)
        .embed(embed) // #normas
        .components(vec![])
}