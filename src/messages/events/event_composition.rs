use duration_string::DurationString;
use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, ModalInteraction};
use crate::events::{EventKind, EventRole};
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::BotInteractionMessage;
use crate::messages::events::{EventScope};
use crate::prelude::*;

#[derive(Clone)]
pub(crate) struct EventComposition {
    confirm: String,
    modify: String,
    modify_confirm: String,
    kind: EventKind
}

impl EventComposition {
    pub(crate) fn new(kind: EventKind, pipeline: &mut InteractionPipeline) -> Self {
        let comp = EventComposition {
            confirm: format!("{kind}_event_comp_confirm"),
            modify: format!("{kind}_event_comp_modify"),
            modify_confirm: format!("{kind}_event_comp_modify_confirm"),
            kind
        };

        pipeline.add(&comp.modify, comp.clone());

        for role in comp.kind.roles() {
            pipeline.add(comp.role_comp_id(role), comp.clone());
            pipeline.add(comp.role_comp_select_id(role), comp.clone());
        }

        let scope = EventScope::new(kind, pipeline);
        pipeline.add(&comp.modify_confirm, scope.clone());
        pipeline.add(&comp.confirm, scope);

        comp
    }

    fn role_comp_id(&self, r: EventRole) -> String {
        format!("{}_comp_{}", self.kind, r.to_id())
    }

    fn role_comp_select_id(&self, r: EventRole) -> String {
        format!("{}_comp_select_{}", self.kind, r.to_id())
    }

    fn button_comp(&self) -> CreateActionRow {
        CreateActionRow::Buttons(self.kind.roles()
            .into_iter()
            .filter_map(|role| match role {
                EventRole::Reserve | EventRole::Absent => None,
                _ => Some(role.to_button(self.role_comp_id(role), role.to_string()))
            }).collect()
        )
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for EventComposition {
    async fn modal(&self, interaction: &ModalInteraction, _ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let title = get_input_value(&interaction.data.components, 0).unwrap();
        let duration = get_input_value(&interaction.data.components, 1)
            .unwrap().parse::<DurationString>().unwrap();
        let description = get_input_value(&interaction.data.components, 2).unwrap();

        let event = store.create_event(
            interaction.message.as_ref().unwrap().id,
            title, description, duration, self.kind, interaction.user.id
        ).await?;

        let response = CreateInteractionResponseMessage::new()
            .add_embed(event.embed_preview())
            .add_embed(CreateEmbed::new()
                .title("Composicion por defecto")
                .fields(event.roles.iter().filter_map(|pr| {
                    match pr.role {
                        EventRole::Reserve | EventRole::Absent => None,
                        _ => Some((pr.role.to_string(), pr.max.map(|max| max.to_string()).unwrap_or("N/A".to_string()), true))
                    }
                })))
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new(&self.confirm)
                    .label("Confirmar")
                    .style(ButtonStyle::Success),
                CreateButton::new(&self.modify)
                    .label("Modificar")
                    .style(ButtonStyle::Secondary)
            ])]);

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }

    async fn component(&self, interaction: &ComponentInteraction, _ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let mut components = if let Some(role) = self.kind.roles().into_iter()
            .find(|r| self.role_comp_id(*r) == interaction.data.custom_id) {
            let kind = CreateSelectMenuKind::String {
                options: (1..12)
                    .map(|n| CreateSelectMenuOption::new(n.to_string(), n.to_string()))
                    .collect()
            };
            vec![
                CreateActionRow::SelectMenu(CreateSelectMenu::new(self.role_comp_select_id(role), kind)),
                self.button_comp()
            ]
        } else {
            if let Some(role) = self.kind.roles().into_iter()
                .find(|r| self.role_comp_select_id(*r) == interaction.data.custom_id) {
                let value = get_selected_option(interaction).map(|n| n.parse::<usize>().ok()).flatten().unwrap();
                store.update_role_max(interaction.message.id, role, value).await?;
            }
            vec![self.button_comp()]
        };

        components.push(CreateActionRow::Buttons(vec![
            CreateButton::new(&self.modify_confirm)
                .label("Continuar")
                .style(ButtonStyle::Secondary)
        ]));

        let event = store.get_event(interaction.message.id).await?;
        Ok(CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .components(components)
        ))
    }
}