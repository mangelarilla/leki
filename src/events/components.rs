use serenity::all::{ButtonStyle, ChannelType, CreateActionRow, CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EmojiId, ReactionType};

pub(crate) fn new_event_components(trial_id: &str, pvp_id: &str, generic_id: &str) -> Vec<CreateActionRow> {
    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(trial_id).label("Trial").style(ButtonStyle::Secondary),
        CreateButton::new(pvp_id).label("PvP").style(ButtonStyle::Secondary),
        CreateButton::new(generic_id).label("Generico").style(ButtonStyle::Secondary)
    ])]
}

pub fn event_scope_components(id_public: &str, id_semi_public: &str, id_private: &str) -> Vec<CreateActionRow> {
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

pub(crate) fn event_components_backup() -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        CreateButton::new("signup_reserve")
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

pub(crate) fn select_player_class(id: &str) -> Vec<CreateActionRow> {
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

    vec![
        CreateActionRow::SelectMenu(CreateSelectMenu::new(id, class_selector)
            .max_values(1)
            .placeholder("Selecciona clase"))
    ]
}

fn class_option(label: &str, description: &str, emoji_id: u64, emoji_label: &str) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(label, format!("<:{}:{}>", emoji_label, emoji_id))
        .description(description)
        .emoji(ReactionType::Custom {animated: false, id: EmojiId::new(emoji_id), name: Some(emoji_label.to_string())})
}