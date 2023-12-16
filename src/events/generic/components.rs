use serenity::all::{ButtonStyle, CreateActionRow, CreateButton};
use crate::events::components::event_components_backup;
use crate::prelude::components::*;

pub fn event_generic_basic_info() -> Vec<CreateActionRow> {
    vec![
        short_input("Titulo del evento", "ev_generic_title", "Aventuras en poletas", true),
        short_input("Duracion", "ev_generic_duration", "1h", true),
        long_input("Descripción", "ev_generic_description", "Se empezara a montar 10 minutos antes\nbla bla bla", true),
    ]
}

pub fn event_generic_signup_components(id: &str) -> Vec<CreateActionRow> {
    let signup = CreateActionRow::Buttons(vec![
        CreateButton::new(id)
            .label("Apuntarse")
            .style(ButtonStyle::Success),
    ]);

    vec![signup, event_components_backup()]
}