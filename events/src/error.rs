#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DurationParse(String),
    #[error(transparent)]
    Serenity(#[from] serenity::prelude::SerenityError),
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Timestamp(#[from] serenity::model::timestamp::InvalidTimestamp),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Postgres(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("Role `{0}` is full")]
    RoleFull(String),
    #[error("Interaction not registered `{0}`")]
    UnknownInteraction(String),
    #[error("EventRole not registered `{0}`")]
    UnknownRole(String),
    #[error("PlayerClass not registered `{0}`")]
    UnknownClass(String),
    #[error("Parse event error: `{0}`")]
    ParseEvent(String),
    #[error("Interaction Timeout")]
    Timeout,
    #[error("Not a day channel: `{0}`")]
    NotDay(String),
}