use serenity::builder::CreateEmbed;
use crate::prelude::embeds::*;

pub fn new_event_embed() -> CreateEmbed {
    basic("Nuevo evento", "Elige tipo de evento")
}