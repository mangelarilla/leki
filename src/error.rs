
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
    #[error(transparent)]
    Serenity(#[from] serenity::prelude::SerenityError),
    #[error(transparent)]
    Timestamp(#[from] serenity::model::timestamp::InvalidTimestamp),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError)
}