#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DurationParse(#[from] anyhow::Error),
    #[error(transparent)]
    Serenity(#[from] serenity::prelude::SerenityError),
    #[error(transparent)]
    Timestamp(#[from] serenity::model::timestamp::InvalidTimestamp),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Role `{0}` is full")]
    RoleFull(String),
    #[error("Interaction not registered `{0}`")]
    UnknownInteraction(String)
}