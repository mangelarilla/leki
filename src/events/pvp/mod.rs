pub mod models;
pub(crate) mod components;
pub(crate) mod embeds;

#[derive(Copy, Clone, Debug)]
pub enum PvPRole {
    Tank, Healer, Brawler, Bomber, Ganker
}