use serenity::all::{ComponentInteraction, Context, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, Interaction, Message};
use crate::events::Event;
use crate::prelude::*;

pub(super) async fn select_role(message: &Message, interaction: &Interaction, ctx: &Context, event: &mut Event) -> Result<ComponentInteraction> {
    match interaction {
        Interaction::Command(i) => i.create_response(&ctx.http, interaction_response(event)).await?,
        Interaction::Component(i) => i.create_response(&ctx.http, interaction_response(event)).await?,
        Interaction::Modal(i) => i.create_response(&ctx.http, interaction_response(event)).await?,
        _ => {}
    }

    if let Some(interaction) = message.await_component_interaction(&ctx.shard).await {
        event.notification_role = get_selected_role(&interaction);
        Ok(interaction)
    } else {
        Err(Error::Timeout)
    }
}

fn interaction_response(event: &Event) -> CreateInteractionResponse {
    CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event.embed_preview())
            .select_menu(CreateSelectMenu::new("create_event_role_select", CreateSelectMenuKind::Role {
                default_roles: None
            }).placeholder("Rol de roster (avanzado, basico...)"))
            .button(CreateButton::new("create_event_role_confirm").label("Continuar"))
    )
}