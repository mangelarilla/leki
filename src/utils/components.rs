use serenity::all::{CreateActionRow, CreateInputText, InputTextStyle};

pub(crate) fn short_input(label: &str, id: &str, placeholder: &str, required: bool) -> CreateActionRow {
    CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, label, id)
        .placeholder(truncate(placeholder, 50))
        .required(required))
}

pub(crate) fn long_input(label: &str, id: &str, placeholder: &str, required: bool) -> CreateActionRow {
    CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, label, id)
        .placeholder(truncate(placeholder, 80))
        .required(required))
}

fn truncate(s: &str, max_chars: usize) -> String {
    let s = match s.char_indices().nth(max_chars - 3) {
        None => s,
        Some((idx, _)) => &s[..idx],
    };

    format!("{s}...")
}