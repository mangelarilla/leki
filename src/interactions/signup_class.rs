use std::sync::Arc;
use serenity::builder::CreateComponents;
use serenity::model::id::UserId;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::Context;
use crate::prelude::*;

pub(crate) async fn dd(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    handle(ctx, interaction, |d| &mut d.dds).await
}
pub(crate) async fn tank(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    handle(ctx, interaction, |d| &mut d.tanks).await
}
pub(crate) async fn healer(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    handle(ctx, interaction, |d| &mut d.healers).await
}

async fn handle<F>(ctx: &Context, interaction: &MessageComponentInteraction, signup_role: F) -> Result<()>
    where F: FnOnce(&mut TrialData) -> &mut Vec<(String, UserId)>
{
    let class = interaction.data.values.first().unwrap();
    let reference = interaction.message.message_reference.clone().unwrap();
    let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;
    let mut data = parse_trial_data(&original_msg).unwrap();
    remove_from_all_roles(&mut data, interaction.user.id);
    let selected_role = signup_role(&mut data);
    selected_role.push((class.to_string(), interaction.user.id));
    original_msg.edit(&ctx.http, |msg| msg
        .set_embed(event_embed(&data))
    ).await?;
    crate::tasks::set_reminder(data.datetime.unwrap(), Arc::new(ctx.clone()), interaction.user.id);
    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(InteractionResponseType::UpdateMessage)
        .interaction_response_data(|d| d
            .embed(|e| e.description("Ya estas dentro!"))
            .set_components(CreateComponents::default())
        )
    ).await?;
    Ok(())
}