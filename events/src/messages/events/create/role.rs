use serenity::all::{ComponentInteraction, Context, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, Message};
use crate::events::Event;
use crate::prelude::*;

pub(super) async fn select_role(message: &Message, interaction: &ComponentInteraction, ctx: &Context, event: &mut Event) -> Result<ComponentInteraction> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .select_menu(CreateSelectMenu::new("create_event_role_select", CreateSelectMenuKind::Role {
                default_roles: None
            }).placeholder("Rol de roster (avanzado, basico...)"))
            .button(CreateButton::new("create_event_role_confirm").label("Continuar"))
    )).await?;

    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        event.notification_role = get_selected_role(&interaction);
        Ok(interaction)
    } else {
        Err(Error::Timeout)
    }
}