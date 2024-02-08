use serenity::all::{ButtonStyle, ComponentInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind};
use shuttle_persist::PersistInstance;
use crate::events::{Event, EventKind, EventRole, EventScopes, Player};
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
            pipeline.add(scope.role_scope_id(role, EventScopes::SemiPublic), scope.clone());
            pipeline.add(scope.role_scope_id(role, EventScopes::Private), scope.clone());
        }

        let day = SelectDate::new(kind, pipeline);
        pipeline.add(&scope.id_confirm, day.clone());
        pipeline.add(&scope.id_public, day);

        scope
    }

    fn role_scope_id(&self, r: EventRole, scope: EventScopes) -> String {
        format!("{}_{scope}_{}", self.kind, r.to_id())
    }

    fn role_scope_select_id(&self, r: EventRole, scope: EventScopes) -> String {
        format!("{}_{scope}_select_{}", self.kind, r.to_id())
    }

    fn scope_role_buttons(&self, scope: EventScopes) -> CreateActionRow {
        CreateActionRow::Buttons(self.kind.roles()
            .into_iter()
            .filter_map(|role| match role {
                EventRole::Absent => None,
                _ => Some(role.to_button(self.role_scope_id(role, scope), role.to_string()))
            }).collect()
        )
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
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &PersistInstance) -> Result<CreateInteractionResponse> {
        let mut event = store.load::<Event>(interaction.message.id.to_string().as_str())?;

        let components = if interaction.data.custom_id == self.id_semi_public {
            event.scope = EventScopes::SemiPublic;
            vec![self.scope_role_buttons(EventScopes::SemiPublic), self.scope_confirm()]
        } else if interaction.data.custom_id == self.id_private {
            event.scope = EventScopes::Private;
            vec![self.scope_role_buttons(EventScopes::Private), self.scope_confirm()]
        } else if let Some(role) = self.kind.roles().into_iter()
            .find(|r| self.role_scope_id(*r, event.scope) == interaction.data.custom_id) {
            vec![
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(self.role_scope_select_id(role, event.scope), CreateSelectMenuKind::User {
                        default_users: None
                    }).max_values(12)
                ),
                self.scope_confirm()
            ]
        } else if let Some(role) = self.kind.roles().into_iter()
            .find(|r| self.role_scope_select_id(*r, event.scope) == interaction.data.custom_id) {
            let (mut players, max) = event.roles.remove(&role).unwrap();
            let guild = interaction.guild_id.clone().unwrap();
            for user in get_selected_users(interaction) {
                let member = guild.member(&ctx.http, user).await?;
                players.push(Player::new(user, member.nick.unwrap()));
            }
            event.roles.insert(role, (players, max));
            vec![self.scope_role_buttons(event.scope), self.scope_confirm()]
        } else {
            vec![self.scope_buttons()]
        };

        let response = CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .components(components);

        store.save(interaction.message.id.to_string().as_str(), event)?;

        Ok(CreateInteractionResponse::UpdateMessage(response))
    }
}