use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind};
use crate::events::{EventKind, EventRole, EventScopes, Player};
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::BotInteractionMessage;
use crate::messages::events::{SelectDate};
use crate::prelude::*;

#[derive(Clone)]
pub(crate) struct EventScope {
    kind: EventKind,
    id_public: String,
    id_semi_public: String,
    id_private: String,
    id_confirm: String
}

impl EventScope {
    pub(crate) fn new(kind: EventKind, pipeline: &mut InteractionPipeline) -> Self {
        let scope = Self {
            kind,
            id_public: format!("{kind}_event_public"),
            id_private: format!("{kind}_event_private"),
            id_semi_public: format!("{kind}_event_semi_public"),
            id_confirm: format!("{kind}_event_scope_confirm")
        };

        pipeline.add(&scope.id_semi_public, scope.clone());
        pipeline.add(&scope.id_semi_public, scope.clone());

        for role in kind.roles() {
            pipeline.add(scope.role_scope_id(role), scope.clone());
            pipeline.add(scope.role_scope_select_id(role), scope.clone());
        }

        let day = SelectDate::new(kind, pipeline);
        pipeline.add(&scope.id_confirm, day.clone());
        pipeline.add(&scope.id_public, day);

        scope
    }

    fn role_scope_id(&self, r: EventRole) -> String {
        format!("{}_scope_{}", self.kind, r.to_id())
    }

    fn role_scope_select_id(&self, r: EventRole) -> String {
        format!("{}_scope_select_{}", self.kind, r.to_id())
    }

    fn scope_role_buttons(&self) -> CreateActionRow {
        CreateActionRow::Buttons(self.kind.roles()
            .into_iter()
            .filter_map(|role| match role {
                EventRole::Absent | EventRole::Reserve => None,
                _ => Some(role.to_button(self.role_scope_id(role), role.to_string()))
            }).collect()
        )
    }

    fn scope_reserve_button(&self) -> CreateActionRow {
        let role = EventRole::Reserve;
        CreateActionRow::Buttons(vec![
            role.to_button(self.role_scope_id(role), role.to_string())
        ])
    }

    fn scope_buttons(&self) -> CreateActionRow {
        CreateActionRow::Buttons(vec![
            CreateButton::new(&self.id_public)
                .label("Abierto")
                .style(ButtonStyle::Success),
            CreateButton::new(&self.id_semi_public)
                .label("Semi-abierto")
                .style(ButtonStyle::Secondary),
            CreateButton::new(&self.id_private)
                .label("Cerrado")
                .style(ButtonStyle::Danger)
        ])
    }

    fn scope_confirm(&self) -> CreateActionRow {
        CreateActionRow::Buttons(vec![
            CreateButton::new(&self.id_confirm)
                .label("Continuar")
                .style(ButtonStyle::Secondary)
        ])
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for EventScope {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let components = if interaction.data.custom_id == self.id_semi_public {
            store.update_scope(interaction.message.id, EventScopes::SemiPublic).await?;
            vec![self.scope_role_buttons(), self.scope_reserve_button(), self.scope_confirm()]
        } else if interaction.data.custom_id == self.id_private {
            store.update_scope(interaction.message.id, EventScopes::SemiPublic).await?;
            vec![self.scope_role_buttons(), self.scope_reserve_button(), self.scope_confirm()]
        } else if let Some(role) = self.kind.roles().into_iter()
            .find(|r| self.role_scope_id(*r) == interaction.data.custom_id) {
            vec![
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(self.role_scope_select_id(role), CreateSelectMenuKind::User {
                        default_users: None
                    }).max_values(12)
                ),
                self.scope_confirm()
            ]
        } else if let Some(role) = self.kind.roles().into_iter()
            .find(|r| self.role_scope_select_id(*r) == interaction.data.custom_id) {
            let guild = interaction.guild_id.clone().unwrap();
            for user in get_selected_users(interaction) {
                let member = guild.member(&ctx.http, user).await?;
                store.signup_player(interaction.message.id, role, Player::new(user, member.display_name().to_string())).await?;
            }
            vec![self.scope_role_buttons(), self.scope_reserve_button(), self.scope_confirm()]
        } else {
            vec![self.scope_buttons()]
        };

        let event = store.get_event(interaction.message.id).await?;
        let response = CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .components(components);

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}