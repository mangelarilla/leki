pub mod components;

use serenity::all::ActionRowComponent::InputText;
use serenity::all::{ActionRow, ChannelId, ComponentInteraction, ComponentInteractionDataKind, UserId};

pub fn get_input_value(components: &Vec<ActionRow>, idx: usize) -> Option<String> {
    let input_text = components.get(idx)
        .map(|row| row.components.get(0))
        .flatten().unwrap();

    if let InputText(input) = input_text {
        input.value.clone()
    } else {
        None
    }
}

pub fn get_selected_option(interaction: &ComponentInteraction) -> Option<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        values.first().map(|s| s.to_string())
    } else { None }
}

pub fn get_selected_options(interaction: &ComponentInteraction) -> Vec<String> {
    if let ComponentInteractionDataKind::StringSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
}

pub fn get_selected_users(interaction: &ComponentInteraction) -> Vec<UserId> {
    if let ComponentInteractionDataKind::UserSelect {values} = &interaction.data.kind {
        values.clone()
    } else { vec![] }
}

pub fn get_selected_channel(interaction: &ComponentInteraction) -> Option<ChannelId> {
    if let ComponentInteractionDataKind::ChannelSelect {values} = &interaction.data.kind {
        values.first().map(|c| *c)
    } else { None }
}