use serenity::model::application::component::ActionRowComponent::InputText;
use serenity::model::prelude::component::ActionRow;

pub fn get_text(components: &Vec<ActionRow>, idx: usize) -> String {
    let input_text = components.get(idx)
        .map(|row| row.components.get(0))
        .flatten().unwrap();

    if let InputText(input) = input_text {
        input.value.to_string()
    } else {
        String::new()
    }
}