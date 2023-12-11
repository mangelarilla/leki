use std::sync::Arc;
use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage};
use serenity::builder::CreateEmbed;
use serenity::model::id::UserId;
use serenity::prelude::Context;
use crate::events::trials::models::{parse_trial_data, TrialData};
use crate::prelude::*;

pub(crate) async fn dd(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    handle(ctx, interaction, |d| &mut d.dds).await
}
pub(crate) async fn tank(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    handle(ctx, interaction, |d| &mut d.tanks).await
}
pub(crate) async fn healer(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    handle(ctx, interaction, |d| &mut d.healers).await
}

async fn handle<F>(ctx: &Context, interaction: &ComponentInteraction, signup_role: F) -> Result<()>
    where F: FnOnce(&mut TrialData) -> &mut Vec<(String, UserId)>
{
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        let class = values.first().unwrap();
        let reference = interaction.message.message_reference.clone().unwrap();
        let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;
        let mut data = parse_trial_data(&original_msg).unwrap();
        remove_from_all_roles(&mut data, interaction.user.id);
        let selected_role = signup_role(&mut data);
        selected_role.push((class.to_string(), interaction.user.id));
        original_msg.edit(&ctx.http, EditMessage::new().embed(event_embed(&data))).await?;
        crate::tasks::set_reminder(data.datetime.unwrap(), Arc::new(ctx.clone()), interaction.user.id);
        interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(CreateEmbed::new().description("Ya estas dentro!"))
                .components(vec![])
        )).await?;
    }

    Ok(())
}