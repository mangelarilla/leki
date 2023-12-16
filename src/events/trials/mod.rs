pub mod models;
pub(crate) mod embeds;
pub(crate) mod components;

#[derive(Copy, Clone, Debug)]
pub enum TrialRole {
    Tank, DD, Healer
}