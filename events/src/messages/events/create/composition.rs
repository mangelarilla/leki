use std::time::Duration;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use flate2::read::DeflateDecoder;
use serenity::all::{ButtonStyle, Context, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Interaction, Message, ModalInteraction};
use crate::events::{Event, EventRole, PlayersInRole};
use crate::prelude::*;
use serenity::futures::StreamExt;
use std::io::prelude::*;
use crate::prelude::components::long_input;

pub(super) async fn handle_composition(message: &Message, modal: &ModalInteraction, ctx: &Context, event: &mut Event) -> Result<Interaction> {
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
                    return Ok(Interaction::Component(interaction));
                }
            }
        }

        if interaction.data.custom_id.contains("import") {
            interaction.create_response(&ctx, create_event_import_composition()).await?;
            if let Some(interaction) = message.await_modal_interaction(&ctx).await {
                let code = get_input_value(&interaction.data.components, 0).unwrap();
                let decoded = BASE64_STANDARD.decode(code.as_bytes())?;
                let mut decoder = DeflateDecoder::new(decoded.as_slice());
                let mut json_roles = String::new();
                decoder.read_to_string(&mut json_roles)?;
                let roles: Vec<PlayersInRole> = serde_json::from_str(&json_roles)?;

                event.roles = roles;
                return Ok(Interaction::Modal(interaction))
            }
        }

        Ok(Interaction::Component(interaction))
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
            .button(CreateButton::new("create_event_composition_import")
                .label("Importar desde codigo")
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

fn create_event_import_composition() -> CreateInteractionResponse {
    CreateInteractionResponse::Modal(
        CreateModal::new("create_event_composition_import_modal", "Importar codigo de plantilla")
            .components(vec![
                long_input("Codigo de plantilla", "roster_code", "ec86e8eca854b02f43fb69d63f15e53d", true)
            ])
    )
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