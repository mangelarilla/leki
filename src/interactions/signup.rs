use serenity::all::{ComponentInteraction, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateSelectMenuKind, CreateSelectMenuOption, EmojiId};
use serenity::builder::CreateSelectMenu;
use serenity::client::Context;
use serenity::model::channel::ReactionType;
use serenity::model::id::UserId;
use crate::prelude::*;

pub(crate) async fn dd(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let handled = handle_if_max(ctx, interaction, |d| &mut d.dds, |d| d.max_dds.into()).await?;
    if !handled {
        handle(ctx, interaction, "dd_class").await?;
    }
    Ok(())
}

pub(crate) async fn tank(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let handled = handle_if_max(ctx, interaction, |d| &mut d.tanks, |d| d.max_tanks.into()).await?;
    if !handled {
        handle(ctx, interaction, "tank_class").await?;
    }
    Ok(())
}

pub(crate) async fn healer(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let handled = handle_if_max(ctx, interaction, |d| &mut d.healers, |d| d.max_healers.into()).await?;
    if !handled {
        handle(ctx, interaction, "healer_class").await?;
    }
    Ok(())
}

pub(crate) async fn reserve(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let mut data = parse_trial_data(&interaction.message)?;
    remove_from_all_roles(&mut data, interaction.user.id);
    crate::tasks::unset_reminder(&interaction.user.id);
    data.reserves.push(interaction.user.id);

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event_embed(&data))
    )).await?;
    Ok(())
}

pub(crate) async fn absent(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let mut data = parse_trial_data(&interaction.message)?;
    remove_from_all_roles(&mut data, interaction.user.id);
    crate::tasks::unset_reminder(&interaction.user.id);
    data.absents.push(interaction.user.id);

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(event_embed(&data))
    )).await?;
    Ok(())
}

async fn handle_if_max<F, J>(ctx: &Context, interaction: &ComponentInteraction, signup_role: F, role_max: J) -> Result<bool>
    where F: FnOnce(&mut TrialData) -> &mut Vec<(String, UserId)>,
          J: FnOnce(&TrialData) -> usize,
{
    let mut data = parse_trial_data(&interaction.message).unwrap();
    remove_from_all_roles(&mut data, interaction.user.id);
    let role = signup_role(&mut data);
    if role.len() == role_max(&data) {
        data.reserves.push(interaction.user.id);
        interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(event_embed(&data))
        )).await?;
        interaction.create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
            .ephemeral(true)
            .embed(CreateEmbed::new().description("Rol lleno, se te ha movido a reserva!"))
        ).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn handle(ctx: &Context, interaction: &ComponentInteraction, custom_id: &str) -> Result<()> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .components(vec![
                CreateActionRow::SelectMenu(CreateSelectMenu::new(custom_id, CreateSelectMenuKind::String {
                    options: vec![
                        CreateSelectMenuOption::new("Necro", "<:necro:1154088177796137030>")
                            .description("Nigromante")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1154088177796137030), name: Some("necro".to_string())}),
                        CreateSelectMenuOption::new("Warden", "<:warden:1154134387546398720>")
                            .description("Custodio")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1154134387546398720), name: Some("warden".to_string())}),
                        CreateSelectMenuOption::new("Dragonknight", "<:dk:1157391862659809280>")
                            .description("Caballero dragon")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1157391862659809280), name: Some("dk".to_string())}),
                        CreateSelectMenuOption::new("Sorc", "<:sorc:1157391866971566100>")
                            .description("Brujo")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1157391866971566100), name: Some("sorc".to_string())}),
                        CreateSelectMenuOption::new("Nightblade", "<:nb:1157391864954093608>")
                            .description("Hoja de la noche")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1157391864954093608), name: Some("nb".to_string())}),
                        CreateSelectMenuOption::new("Templar", "<:templar:1157391868850618388>")
                            .description("Templario")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1157391868850618388), name: Some("templar".to_string())}),
                        CreateSelectMenuOption::new("Arcanist", "<:arcanist:1154134563392606218>")
                            .description("Arcanista")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(1154134563392606218), name: Some("arcanist".to_string())})

                    ]
                }).max_values(1).placeholder("Selecciona clase"))
            ])
    )).await?;
    Ok(())
}