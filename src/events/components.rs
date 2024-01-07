use serenity::all::{ButtonStyle, ChannelId, ChannelType, CreateActionRow, CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EmojiId, ReactionType};
use crate::events::models::EventRole;

pub(crate) fn new_event_components(trial_id: impl Into<String>, pvp_id: impl Into<String>, generic_id: impl Into<String>) -> Vec<CreateActionRow> {
    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(trial_id).label("Trial").style(ButtonStyle::Secondary),
        CreateButton::new(pvp_id).label("PvP").style(ButtonStyle::Secondary),
        CreateButton::new(generic_id).label("Generico").style(ButtonStyle::Secondary)
    ])]
}

pub(crate) fn event_scope_components(id_public: impl Into<String>, id_semi_public: impl Into<String>, id_private: impl Into<String>) -> Vec<CreateActionRow> {
    vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new(id_public)
                .label("Abierto")
                .style(ButtonStyle::Success),
            CreateButton::new(id_semi_public)
                .label("Semi-abierto")
                .style(ButtonStyle::Secondary),
            CreateButton::new(id_private)
                .label("Cerrado")
                .style(ButtonStyle::Danger)
        ])
    ]
}

pub(crate) fn event_comp_defaults_components(id_confirm: impl Into<String>, id_change: impl Into<String>) -> Vec<CreateActionRow> {
    vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new(id_confirm)
                .label("Confirmar")
                .style(ButtonStyle::Success),
            CreateButton::new(id_change)
                .label("Modificar")
                .style(ButtonStyle::Secondary)
        ])
    ]
}

pub(crate) fn event_components_backup(id_reserve: &str) -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        CreateButton::new(id_reserve)
            .label("Reserva")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("👋".to_string())),
        CreateButton::new("signup_absent")
            .label("Ausencia")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("❌".to_string()))
    ])
}

pub(crate) fn select_event_channel(id: &str) -> Vec<CreateActionRow> {
    let channel_selector = CreateSelectMenuKind::Channel {
        channel_types: Some(vec![ChannelType::Text]),
        default_channels: None,
    };

    vec![
        CreateActionRow::SelectMenu(
            CreateSelectMenu::new(id, channel_selector)
                .max_values(5)
                .placeholder("Canales del evento")
        )
    ]
}

pub(crate) fn select_time(id: &str, selected_days: &Vec<(ChannelId, String)>) -> Vec<CreateActionRow> {
    let builder = selected_days
        .into_iter()
        .map(|(channel, day)| {
            CreateActionRow::SelectMenu(
                CreateSelectMenu::new(format!("{}__{}_{}", id, channel, day), time_options())
                    .placeholder(format!("Selecciona hora para el dia {}", day))
            )
        })
        .collect();
    builder
}

pub(crate) fn select_player_class(id: impl Into<String>) -> CreateActionRow {
    let class_selector = CreateSelectMenuKind::String {
        options: vec![
            class_option("Arcanist", "Arcanista", 1154134563392606218, "arcanist"),
            class_option("Necro", "Nigromante", 1154088177796137030, "necro"),
            class_option("Warden", "Custodio", 1154134387546398720, "warden"),
            class_option("Dragonknight", "Caballero dragon", 1157391862659809280, "dk"),
            class_option("Templar", "Templario", 1157391868850618388, "templar"),
            class_option("Sorc", "Brujo", 1157391866971566100, "sorc"),
            class_option("Nightblade", "Hoja de la noche", 1157391864954093608, "nb"),
        ]
    };


    CreateActionRow::SelectMenu(CreateSelectMenu::new(id, class_selector)
        .max_values(1)
        .placeholder("Selecciona clase"))
}

pub(crate) fn select_flex_roles(id: impl Into<String>, roles: &[EventRole]) -> CreateActionRow {
    let role_selector = CreateSelectMenuKind::String {
        options: roles.into_iter().map(|r|
            CreateSelectMenuOption::new(r.to_string(), r.to_string())
        ).collect()
    };

    CreateActionRow::SelectMenu(CreateSelectMenu::new(id, role_selector)
        .placeholder("(Opcional) Roles de reserva")
        .max_values(roles.len() as u8))
}

pub(crate) fn time_options() -> CreateSelectMenuKind {
    CreateSelectMenuKind::String {
        options: vec![
            time_option("16:00"), time_option("16:30"),
            time_option("17:00"), time_option("17:30"),
            time_option("18:00"), time_option("18:30"),
            time_option("19:00"), time_option("19:30"),
            time_option("20:00"), time_option("20:30"),
            time_option("21:00"), time_option("21:30"),
            time_option("22:00"), time_option("22:30"),
            time_option("23:00"), time_option("23:30"),
        ]
    }
}

fn time_option(time: &str) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(time, time)
}

fn class_option(label: &str, description: &str, emoji_id: u64, emoji_label: &str) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(label, format!("<:{}:{}>", emoji_label, emoji_id))
        .description(description)
        .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(emoji_id), name: Some(emoji_label.to_string())})
}