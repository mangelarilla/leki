pub mod models;

use chrono::Weekday;
use duration_string::DurationString;
use serenity::all::{ActionRow, Colour, CreateActionRow, CreateEmbed, CreateEmbedAuthor, CreateInputText, CreateInteractionResponseMessage, CreateModal, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, InputTextStyle, Mention, UserId};
use crate::error::Error;
use crate::prelude::*;

pub fn data(id: &str) -> CreateModal {
    CreateModal::new(id, "Información de la trial")
        .components(vec![
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Titulo de la trial", "trial_title")
                .placeholder("Trial nivel avanzado - vRG")
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
        ])
}

pub fn select_date(id: &str, components: &Vec<ActionRow>, leader: &UserId) -> Result<CreateInteractionResponseMessage> {
    let title = get_text(components, 0);
    let duration = get_text(components, 1).parse::<DurationString>()
        .map_err(|e| Error::DurationParse(anyhow::Error::msg(e)))?;
    let description = get_text(components, 2);
    let addons = get_text(components, 3);
    let guides = get_text(components, 4);

    Ok(CreateInteractionResponseMessage::new()
        .embed(preview_embed(&title, &description, duration, leader, &addons, &guides))
        .components(vec![CreateActionRow::SelectMenu(
            CreateSelectMenu::new(id, CreateSelectMenuKind::String {
                options: vec![
                    CreateSelectMenuOption::new("Lunes", "Monday"),
                    CreateSelectMenuOption::new("Martes", "Tuesday"),
                    CreateSelectMenuOption::new("Miercoles", "Wednesday"),
                    CreateSelectMenuOption::new("Jueves", "Thursday"),
                    CreateSelectMenuOption::new("Viernes", "Friday"),
                    CreateSelectMenuOption::new("Sabado", "Saturday"),
                    CreateSelectMenuOption::new("Domingo", "Sunday")
                ]
            })
                .max_values(5)
                .placeholder("Dias de la semana")
        )]))
}

pub fn select_time(id: &str, selected_days: &Vec<String>) -> CreateModal {
    CreateModal::new(id, "Horas del evento")
        .components(selected_days.into_iter().map(|day| {
            let localized = to_weekday_localized(&day.parse::<Weekday>().unwrap());
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, &localized, &localized)
                .placeholder("18:00")
                .required(true))
        }).collect())
}

fn preview_embed(
    title: &str,
    description: &str,
    duration: DurationString,
    leader: &UserId,
    addons: &str,
    guides: &str
) -> CreateEmbed {
    CreateEmbed::new()
        .author(CreateEmbedAuthor::new("Previsualizacion"))
        .title(title)
        .description(description)
        .field(":date: Fecha y Hora:", "", true)
        .field(":hourglass_flowing_sand: Duración", duration, true)
        .field(":crown: Lider", Mention::User(*leader).to_string(), true)
        .field("Guias:", addons, false)
        .field("AddOns recomendados:", guides, false)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field("<:tank:1154134006036713622> Tanks (0/2)", "", false)
        .field("<:dd:1154134731756150974> DD (0/8)", "", false)
        .field("<:healer:1154134924153065544> Healers (0/2)", "", false)
        .field(":wave: Reservas (0)", "", false)
        .field(":x: Ausencias (0)", "", false)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
        .color(Colour::from_rgb(0, 255, 0))
}