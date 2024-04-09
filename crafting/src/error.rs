#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Gear Set: {0}")]
    InvalidGearSet(String),
    #[error(transparent)]
    Serenity(#[from] serenity::prelude::SerenityError),
    #[error("Interaction Timeout")]
    Timeout
}