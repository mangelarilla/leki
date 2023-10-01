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
    where F: FnOnce(&mut TrialData) -> &mut Vec<(String, String)>
{
    let class = interaction.data.values.first().unwrap();
    let reference = interaction.message.message_reference.clone().unwrap();
    let mut original_msg = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await?;
    let mut data = parse_trial_data(&original_msg).unwrap();
    signup_role(&mut data).push((class.to_string(), interaction.user.name.to_string()));

    original_msg.edit(&ctx.http, |msg| msg
        .set_embed(event_embed(&data))
    ).await?;
    Ok(())
}