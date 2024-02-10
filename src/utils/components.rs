use serenity::all::{CreateActionRow, CreateInputText, InputTextStyle};

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