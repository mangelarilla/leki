pub mod models;
pub mod signup;

use duration_string::DurationString;
use serenity::all::{ActionRow, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponseMessage, CreateModal, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EmojiId, InputTextStyle, ReactionType, UserId};
use crate::error::Error;
use crate::events::{event_components_backup, event_embed_backup, event_embed_basic, preview_embed_basic, select_days};
use crate::events::trials::models::TrialData;
use crate::prelude::*;

pub enum TrialRole {
    Tank, DD, Healer
}

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
        .components(vec![CreateActionRow::SelectMenu(select_days(id))]))
}

fn preview_embed(
    title: &str,
    description: &str,
    duration: DurationString,
    leader: &UserId,
    addons: &str,
    guides: &str
) -> CreateEmbed {
    preview_embed_basic(title, description, duration, leader)
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
}

pub fn trial_embed(
    data: &TrialData,
) -> CreateEmbed {
    let embed = event_embed_basic(data)
        .field("Guias:", &data.addons, false)
        .field("AddOns recomendados:", &data.guides, false)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field(
            format!("<:tank:1154134006036713622> Tanks ({}/{})", &data.tanks.len(), &data.max_tanks),
            &data.tanks.iter().map(|(class, player)| format!("└{class} <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field(
            format!("<:dd:1154134731756150974> DD ({}/{})", &data.dds.len(), &data.max_dds),
            &data.dds.iter().map(|(class, player)| format!("└{class} <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field(
            format!("<:healer:1154134924153065544> Healers ({}/{})", &data.healers.len(), &data.max_healers),
            &data.healers.iter().map(|(class, player)| format!("└{class} <@{player}>")).collect::<Vec<String>>().join("\n"),
            false);
    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
}

pub fn trial_components() -> Vec<CreateActionRow> {
    let class_row = CreateActionRow::Buttons(vec![
        CreateButton::new("signup_tank")
            .label("Tank")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())}),
        CreateButton::new("signup_dd")
            .label("DD")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())}),
        CreateButton::new("signup_healer")
            .label("Healer")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())})
    ]);

    vec![class_row, event_components_backup()]
}

pub fn trial_class(id: &str) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .components(vec![
            CreateActionRow::SelectMenu(CreateSelectMenu::new(id, CreateSelectMenuKind::String {
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
}