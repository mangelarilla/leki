use serenity::client::Context;
use serenity::model::application::component::{ButtonStyle};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;

pub(crate) async fn handle(ctx: &Context, command: ApplicationCommandInteraction) {
    command.create_interaction_response(&ctx.http, |r| r
        .interaction_response_data(|d| d
            .embed(|e| e.title("Gestion de eventos"))
            .ephemeral(true)
            .components(|c| c
                .create_action_row(|r| r
                    .create_button(|b| b
                        .custom_id("create_event")
                        .label("Crear evento")
                        .style(ButtonStyle::Success)
                    )
                    .create_button(|b| b
                        .custom_id("update_event")
                        .label("Modificar evento")
                        .style(ButtonStyle::Primary)
                    )
                    .create_button(|b| b
                        .custom_id("delete_event")
                        .label("Borrar evento")
                        .style(ButtonStyle::Danger)
                    )
                )
            )
        )
    ).await.unwrap();
}