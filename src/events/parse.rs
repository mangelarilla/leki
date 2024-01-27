use std::str::FromStr;
use duration_string::DurationString;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::{ActionRow, EmbedField, UserId};
use tracing::instrument;
use crate::events::{EventSignedRole, Player, PlayersInRole};
use crate::prelude::get_input_value;

#[instrument]
pub(crate) fn parse_players_in_role(field: &EmbedField) -> PlayersInRole {
    let players = field.value.clone().lines()
        .filter(|s| !s.is_empty())
        .filter_map(|s| parse_player(s))
        .collect();
    let max = get_max(&field.name).map(|max| max.parse::<usize>().ok()).flatten();
    PlayersInRole::new(players, max)
}

#[instrument]
pub(crate) fn get_max(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".+\/(?P<max>\d+)").unwrap();
    }
    RE.captures(text).and_then(|cap| {
        cap.name("max").map(|max| max.as_str().to_string())
    })
}

pub(super) fn parse_basic_from_modal(components: &Vec<ActionRow>) -> (String, String, DurationString) {
    let title = get_input_value(components, 0);
    let duration = get_input_value(components, 1).unwrap().parse::<DurationString>()
        .unwrap();
    let description = get_input_value(components, 2);

    (title.unwrap(), description.unwrap(), duration)
}

#[instrument]
pub fn parse_player(text: &str) -> Option<Player> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<class><:.+>)*\s*<@(?P<player>\d+)>(?:\s\(Flex:\s)*(?P<flex_roles>.+[^\)])*").unwrap();
    }

    RE.captures(text).and_then(|cap| {
        let class = cap.name("class")
            .map(|max| max.as_str().to_string());
        let user = cap.name("player")
            .map(|max| max.as_str().parse::<u64>().map(|m| UserId::new(m)).ok())
            .flatten();

        let flex_roles = cap.name("flex_roles")
            .map(|roles| roles.as_str()
                .split(",")
                .filter_map(|r| EventSignedRole::from_str(r).ok())
                .collect()
            )
            .unwrap_or(vec![]);

        user.map(|user| if let Some(class) = class {
            Player::Class(user, class, flex_roles)
        } else {
            Player::Basic(user)
        })
    })
}

pub fn empty_to_option(text: String) -> Option<String> {
    if text.is_empty() { None } else { Some(text) }
}