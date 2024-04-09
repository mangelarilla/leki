//! Crate prelude
use std::fmt::Display;
use std::str::FromStr;
use serenity::all::{ComponentInteraction, ComponentInteractionDataKind, CreateSelectMenuOption};
use strum::{EnumMessage, IntoEnumIterator};

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn enum_to_options<T: IntoEnumIterator + EnumMessage + Display>() -> Vec<CreateSelectMenuOption> {
    T::iter()
        .map(|i| CreateSelectMenuOption::new(i.to_string(), i.to_string()))
        .collect()
}

pub fn get_selected_gear<T: FromStr>(interaction: &ComponentInteraction) -> Vec<T> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        values.iter().filter_map(|f| T::from_str(f).ok()).collect()
    } else { vec![] }
}