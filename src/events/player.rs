use serenity::all::UserId;
use super::EventSignedRole;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Player {
    Basic(UserId),
    Class(UserId, String, Vec<EventSignedRole>)
}

#[derive(Debug, Clone)]
pub struct PlayersInRole {
    players: Vec<Player>,
    max: Option<usize>
}

impl PlayersInRole {
    pub(crate) fn new(players: Vec<Player>, max: Option<usize>) -> Self {
        PlayersInRole { players, max }
    }
    pub(crate) fn is_role_full(&self) -> bool {
        self.max.is_some_and(|max| max <= self.players.len())
    }

    pub(super) fn remove_from_role(&mut self, user: UserId) {
        let index = self.players.iter().position(|player| <Player as Into<UserId>>::into(player.clone()) == user);
        if let Some(index) = index {
            self.players.remove(index);
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.players.len()
    }
    pub(crate) fn max(&self) -> Option<usize> {
        self.max
    }
    pub(crate) fn as_slice(&self) -> &[Player] {
        self.players.as_slice()
    }
    pub(crate) fn push(&mut self, player: Player) {
        self.players.push(player)
    }
}

impl Into<Vec<Player>> for PlayersInRole {
    fn into(self) -> Vec<Player> {
        self.players
    }
}

impl Into<Vec<UserId>> for PlayersInRole {
    fn into(self) -> Vec<UserId> {
        self.players.iter().map(|p| p.clone().into()).collect()
    }
}

impl Default for PlayersInRole {
    fn default() -> Self {
        PlayersInRole {
            players: vec![], max: None
        }
    }
}

impl Into<UserId> for Player {
    fn into(self) -> UserId {
        match self {
            Player::Basic(user) => user,
            Player::Class(user, _, _) => user
        }
    }
}

