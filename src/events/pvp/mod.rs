use duration_string::DurationString;
use serenity::all::{ActionRow, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponseMessage, CreateModal, EmojiId, InputTextStyle, ReactionType, UserId};
use crate::error::Error;
use crate::events::{event_components_backup, event_embed_backup, event_embed_basic, format_player_class_embed, preview_embed_basic, select_days};
use crate::events::pvp::models::PvPData;
use crate::prelude::get_text;

pub mod models;
pub mod signup;

pub enum PvPRole {
    Tank, Healer, Brawler, Bomber
}

const THUMBNAIL: &'static str = "https://images.uesp.net/9/9e/ON-icon-alliance-Ebonheart.png";

pub fn data(id: &str) -> CreateModal {
    CreateModal::new(id, "Evento PvP")
        .components(vec![
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Titulo del evento", "pvp_title")
                .placeholder("Premade PvP")
                .required(true)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Duracion", "pvp_duration")
                .placeholder("2h")
                .required(true)),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, "Descripción", "pvp_description")
                .placeholder("Requisitos...\nSe empezara a montar 10 minutos antes\nbla bla bla")
                .required(false)),
        ])
}

pub fn select_date(id: &str, components: &Vec<ActionRow>, leader: &UserId) -> crate::prelude::Result<CreateInteractionResponseMessage> {
    let title = get_text(components, 0);
    let duration = get_text(components, 1).parse::<DurationString>()
        .map_err(|e| Error::DurationParse(anyhow::Error::msg(e)))?;
    let description = get_text(components, 2);

    Ok(CreateInteractionResponseMessage::new()
        .embed(preview_pvp_embed(&title, &description, duration, leader))
        .components(vec![CreateActionRow::SelectMenu(select_days(id))]))
}

fn preview_pvp_embed(
    title: &str,
    description: &str,
    duration: DurationString,
    leader: &UserId
) -> CreateEmbed {
    preview_embed_basic(title, description, duration, leader)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field("<:tank:1154134006036713622> Tanks (0/2)", "", false)
        .field("<:dd:1154134731756150974> Brawlers (0)", "", false)
        .field("<:healer:1154134924153065544> Healers (0/2)", "", false)
        .field(":bomb: Bombers (0)", "", false)
        .field(":wave: Reservas (0)", "", false)
        .field(":x: Ausencias (0)", "", false)
        .field("", "\u{200b}", false)
        .thumbnail(THUMBNAIL)
}

pub fn pvp_embed(
    data: &PvPData,
) -> CreateEmbed {
    let embed = event_embed_basic(data)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field(
            format!("<:tank:1154134006036713622> Tanks ({}/{})", &data.tanks.len(), &data.max_tanks),
            format_player_class_embed(&data.tanks),
            false)
        .field(
            format!("<:dd:1154134731756150974> Brawlers ({})", &data.brawlers.len()),
            format_player_class_embed(&data.brawlers),
            false)
        .field(
            format!("<:healer:1154134924153065544> Healers ({}/{})", &data.healers.len(), &data.max_healers),
            format_player_class_embed(&data.healers),
            false)
        .field(
            format!(":bomb: Bombers ({})", &data.bombers.len()),
            format_player_class_embed(&data.bombers),
            false);
    event_embed_backup(data, embed)
        .field("", "\u{200b}", false)
        .thumbnail(THUMBNAIL)
}

pub fn pvp_components() -> Vec<CreateActionRow> {
    let class_row = CreateActionRow::Buttons(vec![
        CreateButton::new("signup_pvp_tank")
            .label("Tank")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())}),
        CreateButton::new("signup_pvp_brawler")
            .label("DD")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())}),
        CreateButton::new("signup_pvp_healer")
            .label("Healer")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())}),
        CreateButton::new("signup_pvp_bomber")
            .label("Bomber")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Unicode("💣".to_string()))
    ]);

    vec![class_row, event_components_backup()]
}