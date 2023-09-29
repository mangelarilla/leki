use serenity::client::Context;
use serenity::model::channel::ReactionType;
use serenity::model::prelude::EmojiId;
use serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use crate::prelude::*;

pub(crate) async fn dd(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    handle(ctx, interaction, "dd_class").await
}

pub(crate) async fn tank(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    handle(ctx, interaction, "dd_class").await
}

pub(crate) async fn healer(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    handle(ctx, interaction, "dd_class").await
}

async fn handle(ctx: &Context, interaction: &MessageComponentInteraction, custom_id: &str) -> Result<()> {
    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(ChannelMessageWithSource)
        .interaction_response_data(|d| d
            .ephemeral(true)
            .components(|c| c.create_action_row(|row| row
                .create_select_menu(|m| m
                    .custom_id(custom_id)
                    .max_values(1)
                    .placeholder("Selecciona clase")
                    .options(|opt| opt
                        .create_option(|opt| opt
                            .label("Necro").description("Nigromante").value("necro")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1154088177796137030), name: Some("necro".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Warden").description("Custodio").value("warden")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1154134387546398720), name: Some("warden".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Dragonknight").description("Caballero dragon").value("dk")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391862659809280), name: Some("dk".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Sorc").description("Brujo").value("sorc")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391866971566100), name: Some("sorc".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Nightblade").description("Hoja de la noche").value("nb")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391864954093608), name: Some("nb".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Templar").description("Templario").value("templar")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1157391868850618388), name: Some("templar".to_string())})
                        )
                        .create_option(|opt| opt
                            .label("Arcanist").description("Arcanista").value("arcanist")
                            .emoji(ReactionType::Custom {animated: false, id: EmojiId(1154134563392606218), name: Some("arcanist".to_string())})
                        )
                    )
                )
            ))
        )
    ).await?;
    Ok(())
}