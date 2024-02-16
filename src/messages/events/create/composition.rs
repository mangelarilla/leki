use std::time::Duration;
use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Message, ModalInteraction};
use crate::events::{Event, EventRole};
use crate::prelude::*;
use serenity::futures::StreamExt;

pub(super) async fn handle_composition(message: &Message, modal: &ModalInteraction, ctx: &Context, event: &mut Event) -> Result<ComponentInteraction> {
    modal.create_response(&ctx.http, create_event_default_composition(&event)).await?;
    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        // Select modify composition
        if interaction.data.custom_id.contains("modify") {
            interaction.create_response(&ctx.http, create_event_change_composition(&event)).await?;

            // Select role to change max
            let mut role_max_change = message
                .await_component_interaction(&ctx.shard)
                .timeout(Duration::from_secs(60 * 5))
                .stream();

            while let Some(interaction) = role_max_change.next().await {
                if let Some(role) = EventRole::from_partial_id(&interaction.data.custom_id) {
                    // Select max
                    if !interaction.data.custom_id.ends_with("select") {
                        interaction.create_response(&ctx.http, create_event_change_composition_select(role, &event)).await?;
                    } else {
                        // Set new max for role
                        let max = get_selected_option(&interaction)
                            .map(|n| n.parse::<usize>().ok())
                            .flatten();

                        event.set_max(role, max);
                        interaction.create_response(&ctx.http, create_event_change_composition(&event)).await?;
                    }
                } else {
                    return Ok(interaction);
                }
            }
        }
        Ok(interaction)
    } else {
        Err(Error::Timeout)
    }
}

fn create_event_default_composition(event: &Event) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embeds(composition_embeds(event))
            .button(CreateButton::new("create_event_composition_confirm")
                .label("Confirmar")
                .style(ButtonStyle::Success))
            .button(CreateButton::new("create_event_composition_modify")
                .label("Modificar")
                .style(ButtonStyle::Secondary))
    )
}

fn composition_embeds(event: &Event) -> Vec<CreateEmbed> {
    vec![
        event.embed_preview(),
        CreateEmbed::new()
            .title("Composicion por defecto")
            .fields(event.roles.iter().filter_map(|pr| {
                match pr.role {
                    EventRole::Reserve | EventRole::Absent => None,
                    _ => Some((pr.role.to_string(), pr.max.map(|max| max.to_string()).unwrap_or("N/A".to_string()), true))
                }
            }))
    ]
}

fn create_event_change_composition(event: &Event) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embeds(composition_embeds(event))
            .components(composition_buttons(event))
    )
}

fn composition_buttons(event: &Event) -> Vec<CreateActionRow> {
    vec![
        CreateActionRow::Buttons(event.kind.roles()
            .into_iter()
            .filter_map(|role| if !role.is_backup_role() {
                Some(role.to_button(format!("create_event_composition_{}", role.to_id()), role.to_string()))
            } else { None }).collect()
        ),
        CreateActionRow::Buttons(vec![
            CreateButton::new("create_event_composition_modify_confirm")
                .label("Continuar")
                .style(ButtonStyle::Secondary)
        ])
    ]
}

fn create_event_change_composition_select(role: EventRole, event: &Event) -> CreateInteractionResponse {
    let kind = CreateSelectMenuKind::String {
        options: (0..12)
            .map(|n| CreateSelectMenuOption::new(n.to_string(), n.to_string()))
            .collect()
    };

    let mut components = composition_buttons(event);
    components.insert(0, CreateActionRow::SelectMenu(
        CreateSelectMenu::new(format!("create_event_composition_{}_select", role.to_id()), kind))
    );

    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .components(components)
    )
}