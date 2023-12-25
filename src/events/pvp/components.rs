use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, EmojiId, ReactionType};
use crate::events::components::event_components_backup;
use crate::prelude::components::*;

pub fn pvp_basic_info() -> Vec<CreateActionRow> {
    vec![
        short_input("Titulo del evento", "pvp_title", "Premade PvP", true),
        short_input("Duracion", "pvp_duration", "2h", true),
        long_input("Descripción", "pvp_description", "Requisitos...\nSe empezara a montar 10 minutos antes\nbla bla bla", true),
    ]
}

pub fn pvp_signup_components() -> Vec<CreateActionRow> {
    let class_row = CreateActionRow::Buttons(vec![
        CreateButton::new("signup_pvp_tank")
            .label("Tank")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())}),
        CreateButton::new("signup_pvp_brawler")
            .label("Brawler")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())}),
        CreateButton::new("signup_pvp_healer")
            .label("Healer")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())}),
        CreateButton::new("signup_pvp_bomber")
            .label("Bomber")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Unicode("💣".to_string())),
        CreateButton::new("signup_pvp_ganker")
            .label("Ganker")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Unicode("🔪".to_string()))
    ]);

    vec![class_row, event_components_backup()]
}

pub(crate) fn pvp_new_comp_components() -> Vec<CreateActionRow> {
    vec![
        short_input("Max Tanks", "pvp_max_tanks", "2 (Por defecto no hay maximo)", false),
        short_input("Max Brawlers", "pvp_max_brawlers", "(Por defecto no hay maximo)", false),
        short_input("Max Healers", "pvp_max_healers", "3 (Por defecto no hay maximo)", false),
        short_input("Max Bombers", "pvp_max_bombers", "(Por defecto no hay maximo)", false),
        short_input("Max Gankers", "pvp_max_gankers", "(Por defecto no hay maximo)", false)
    ]
}

pub fn pvp_participants_components(tanks_id: &str, brawlers_id: &str, healers_id: &str, bombers_id: &str, gankers_id: &str, id: &str) -> Vec<CreateActionRow> {
    vec![
        get_roster_select(tanks_id, "Tanques titulares", 2),
        get_roster_select(brawlers_id, "Brawlers titulares", 8),
        get_roster_select(healers_id, "Healers titulares", 2),
        get_roster_select(bombers_id, "Bombers titulares", 3),
        get_roster_select(gankers_id, "Gankers titulares", 3),
        CreateActionRow::Buttons(vec![
            CreateButton::new(id)
                .label("Continuar")
                .style(ButtonStyle::Primary)
        ])
    ]
}