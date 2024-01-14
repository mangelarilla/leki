use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, EmojiId, ReactionType};
use crate::events::components::event_components_backup;
use crate::prelude::components::*;

pub fn trial_participants_components(tanks_id: &str, dds_id: &str, healers_id: &str, reserves_id: &str, id: &str) -> Vec<CreateActionRow> {
    vec![
        get_roster_select(tanks_id, "Tanques titulares", 2),
        get_roster_select(dds_id, "DD titulares", 8),
        get_roster_select(healers_id, "Healers titulares", 2),
        get_roster_select(reserves_id, "Reservas", 8),
        CreateActionRow::Buttons(vec![
            CreateButton::new(id)
                .label("Continuar")
                .style(ButtonStyle::Primary)
        ])
    ]
}

pub fn trial_basic_info_components() -> Vec<CreateActionRow> {
    vec![
        short_input("Titulo de la trials", "trial_title", "Trial nivel avanzado - vRG", true),
        short_input("Duracion", "trial_duration", "2h", true),
        long_input("Descripción", "trial_description", "Se empezara a montar 10 minutos antes\nbla bla bla", true),
        long_input("AddOns", "trial_addons", "[RaidNotifier](https://esoui.com/RaidNotifier)\n[CodeCombat](https://esoui.com/CodeCombat)", false),
        long_input("Guias", "trial_guides", "[Alcast](https://alcast.com)\n[Xynode](https://xynode.com)", false),
    ]
}

pub(crate) fn trial_new_comp_components() -> Vec<CreateActionRow> {
    vec![
        short_input("Max Tanks", "trial_max_tanks", "2", false),
        short_input("Max DD", "trial_max_dd", "8", false),
        short_input("Max Healers", "trial_max_healers", "2", false)
    ]
}

pub fn trial_signup_components() -> Vec<CreateActionRow> {
    let class_row = CreateActionRow::Buttons(vec![
        CreateButton::new("signup_trial_tank")
            .label("Tank")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())}),
        CreateButton::new("signup_trial_dd")
            .label("DD")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())}),
        CreateButton::new("signup_trial_healer")
            .label("Healer")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())})
    ]);

    vec![class_row, event_components_backup("signup_trial_reserve")]
}