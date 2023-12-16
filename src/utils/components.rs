use serenity::all::{CreateActionRow, CreateInputText, CreateSelectMenu, CreateSelectMenuKind, InputTextStyle};

pub(crate) fn short_input(label: &str, id: &str, placeholder: &str, required: bool) -> CreateActionRow {
    CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, label, id)
        .placeholder(placeholder)
        .required(required))
}

pub(crate) fn long_input(label: &str, id: &str, placeholder: &str, required: bool) -> CreateActionRow {
    CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, label, id)
        .placeholder(placeholder)
        .required(required))
}

pub(crate) fn get_roster_select(id: &str, placeholder: &str, max: u8) -> CreateActionRow {
    CreateActionRow::SelectMenu(
        CreateSelectMenu::new(id, CreateSelectMenuKind::User {
            default_users: None
        }).max_values(max).placeholder(placeholder)
    )
}