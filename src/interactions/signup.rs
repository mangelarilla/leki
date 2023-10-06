use serenity::client::Context;
use serenity::model::channel::ReactionType;
use serenity::model::prelude::{EmojiId, InteractionResponseType};
use serenity::model::prelude::message_component::MessageComponentInteraction;
use crate::prelude::*;

pub(crate) async fn dd(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    let handled = handle_if_max(ctx, interaction, |d| &mut d.dds, |d| d.max_dds.into()).await?;
    if !handled {
        handle(ctx, interaction, "dd_class").await?;
    }
    Ok(())
}

pub(crate) async fn tank(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    let handled = handle_if_max(ctx, interaction, |d| &mut d.tanks, |d| d.max_tanks.into()).await?;
    if !handled {
        handle(ctx, interaction, "tank_class").await?;
    }
    Ok(())
}

pub(crate) async fn healer(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    let handled = handle_if_max(ctx, interaction, |d| &mut d.healers, |d| d.max_healers.into()).await?;
    if !handled {
        handle(ctx, interaction, "healer_class").await?;
    }
    Ok(())
}

pub(crate) async fn reserve(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    let mut data = parse_trial_data(&interaction.message).unwrap();
    remove_from_all_roles(&mut data, &interaction.user.name);

    data.reserves.push(interaction.user.name.to_string());

    interaction.create_interaction_response(&ctx.http, |m| m
        .kind(InteractionResponseType::UpdateMessage)
        .interaction_response_data(|d| d
            .set_embed(event_embed(&data))
        )
    ).await?;
    Ok(())
}

pub(crate) async fn absent(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    let mut data = parse_trial_data(&interaction.message).unwrap();
    remove_from_all_roles(&mut data, &interaction.user.name);

    data.absents.push(interaction.user.name.to_string());

    interaction.create_interaction_response(&ctx.http, |m| m
        .kind(InteractionResponseType::UpdateMessage)
        .interaction_response_data(|d| d
            .set_embed(event_embed(&data))
        )
    ).await?;
    Ok(())
}

async fn handle_if_max<F, J>(ctx: &Context, interaction: &MessageComponentInteraction, signup_role: F, role_max: J) -> Result<bool>
    where F: FnOnce(&mut TrialData) -> &mut Vec<(String, String)>,
          J: FnOnce(&TrialData) -> usize,
{
    let mut data = parse_trial_data(&interaction.message).unwrap();
    remove_from_all_roles(&mut data, &interaction.user.name);
    let role = signup_role(&mut data);
    if role.len() == role_max(&data) {
        data.reserves.push(interaction.user.name.to_string());
        interaction.create_interaction_response(&ctx.http, |m| m
            .kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|d| d
                .set_embed(event_embed(&data))
            )
        ).await?;
        interaction.create_followup_message(&ctx.http, |m| m
            .ephemeral(true)
            .embed(|e| e.description("Rol lleno, se te ha movido a reserva!"))
        ).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn handle(ctx: &Context, interaction: &MessageComponentInteraction, custom_id: &str) -> Result<()> {
    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|d| d
            .ephemeral(true)
            .components(|c| c.create_action_row(|row| row
                .create_select_menu(|m| m
                    .custom_id(custom_id)
                    .max_values(1)
                    .placeholder("Selecciona clase")
                    .options(|opt| opt
                        .create_option(|opt| opt
                            .label("Necro").description("Nigromante").value("<:necro:1154088177796137030>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1154088177796137030), name: Some("necro".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Warden").description("Custodio").value("<:warden:1154134387546398720>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1154134387546398720), name: Some("warden".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Dragonknight").description("Caballero dragon").value("<:dk:1157391862659809280>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391862659809280), name: Some("dk".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Sorc").description("Brujo").value("<:sorc:1157391866971566100>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391866971566100), name: Some("sorc".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Nightblade").description("Hoja de la noche").value("<:nb:1157391864954093608>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391864954093608), name: Some("nb".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Templar").description("Templario").value("<:templar:1157391868850618388>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391868850618388), name: Some("templar".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Arcanist").description("Arcanista").value("<:arcanist:1154134563392606218>")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1154134563392606218), name: Some("arcanist".to_string())})
                        )
                    )
                )
            ))
        )
    ).await?;
    Ok(())
}