use serenity::all::{ComponentInteraction, CreateActionRow, CreateInteractionResponse, CreateModal, InputTextStyle};
use serenity::builder::CreateInputText;
use serenity::prelude::Context;

use crate::prelude::*;
pub(crate) async fn handle(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let modal = CreateModal::new("trial_texts", "Información de la trial")
        .components(vec![
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Titulo de la trial", "trial_title")
                .placeholder("Trial nivel avanzado - vDSR")
                .required(true)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Duracion", "trial_duration")
                .placeholder("2h")
                .required(true)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, "Descripción", "trial_description")
                .placeholder("Se empezara a montar 10 minutos antes\nbla bla bla")
                .required(false)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, "AddOns", "trial_addons")
                .placeholder("[RaidNotifier](https://esoui.com/RaidNotifier)\n[CodeCombat](https://esoui.com/CodeCombat)")
                .required(false)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, "Guias", "trial_guides")
                .placeholder("[Alcast](https://alcast.com)\n[Xynode](https://xynode.com)")
                .required(false)),
        ]);

    interaction.create_response(&ctx.http, CreateInteractionResponse::Modal(modal)).await?;

    Ok(())
}