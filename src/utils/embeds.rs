use serenity::all::{CreateEmbed};



pub(crate) fn basic(title: &str, description: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
}