#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DurationParse(String),
    #[error(transparent)]
    Serenity(#[from] serenity::prelude::SerenityError),
    #[error(transparent)]
    Timestamp(#[from] serenity::model::timestamp::InvalidTimestamp),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Postgres(#[from] sqlx::Error),
    #[error("Role `{0}` is full")]
    RoleFull(String),
    #[error("Interaction not registered `{0}`")]
    UnknownInteraction(String),
    #[error("EventRole not registered `{0}`")]
    UnknownRole(String),
    #[error("PlayerClass not registered `{0}`")]
    UnknownClass(String),
    #[error("Parse event error")]
    ParseEvent,
    #[error("Interaction Timeout")]
    Timeout,
}