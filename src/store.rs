use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{MessageId, ScheduledEventId, UserId};
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use crate::events::{Event, EventKind, EventRole, EventScopes, Player, PlayerClass, PlayersInRole};
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Store {
    pool: PgPool
}

impl Store {
    pub(super) fn new(pool: PgPool) -> Self {
        Store {pool}
    }

    pub async fn get_event(&self, message_id: MessageId) -> Result<Event> {
        let mut event: Event = sqlx::query_as!(DbEvent, r#"
        select
            title,
            kind as "kind!: EventKind",
            scope as "scope!: EventScopes",
            description, datetime, duration, leader, scheduled_event
        from events.events
        where message_id = $1"#, message_id.get() as i64)
            .fetch_one(&self.pool).await?.into();

        let player_roles = sqlx::query_as!(DbPlayerRole, r#"
        select
            message_id,
            role as "role!: EventRole",
            max
        from events.player_roles
        where message_id = $1"#, message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        let players = sqlx::query_as!(DbPlayer, r#"
        select
            message_id,
            role as "role!: EventRole",
            user_id, name,
            class as "class!: PlayerClass"
        from events.players
        where message_id = $1"#, message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        let flex_roles: Vec<DbFlexRole> = sqlx::query_as!(DbFlexRole, r#"
        select
            message_id,
            role as "role!: EventRole"
        from events.flex_roles
        where message_id = $1"#, message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        event.roles = player_roles.into_iter()
            .map(|pr| {
                let msg = pr.message_id;
                let mut pr: PlayersInRole = pr.into();
                pr.players = players.iter()
                    .filter_map(|p| if p.message_id == msg && p.role == pr.role {
                        let mut player: Player = p.into();
                        player.flex = flex_roles.iter()
                            .filter_map(|f| if f.message_id == msg && p.user_id as u64 == player.id.get() {
                                Some(f.role)
                            } else { None })
                            .collect();
                        Some(player)
                    } else { None })
                    .collect();

                pr
            })
            .collect();

        Ok(event)
    }

    pub async fn create_event(&self, message_id: MessageId, title: String, description: String, duration: DurationString, kind: EventKind, leader: UserId) -> Result<Event> {
        sqlx::query!(r#"
        insert into events.events(message_id,kind,scope,title,description,duration,leader)
        values($1,$2,$3,$4,$5,$6,$7)
        "#, message_id.get() as i64, kind as EventKind, EventScopes::Public as EventScopes, title, description, duration.to_string(), leader.get() as i64)
            .execute(&self.pool).await?;

        for role in kind.roles() {
            sqlx::query!(r#"
            insert into events.player_roles(message_id,role,max)
            values($1,$2,$3)
            "#, message_id.get() as i64, role as EventRole, kind.default_role_max(role).map(|m| m as i16))
                .execute(&self.pool).await?;
        }

        self.get_event(message_id).await
    }

    pub async fn update_discord_event(&self, message_id: MessageId, event_id: ScheduledEventId) -> Result<()> {
        sqlx::query!(r#"
        update events.events
        set scheduled_event = $1
        where message_id = $2
        "#, event_id.get() as i64, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_scope(&self, message_id: MessageId, scope: EventScopes) -> Result<()> {
        sqlx::query!(r#"
        update events.events
        set scope = $1
        where message_id = $2
        "#, scope as EventScopes, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_datetime(&self, message_id: MessageId, datetime: DateTime<Utc>) -> Result<()> {
        let datetime = OffsetDateTime::from_unix_timestamp(datetime.timestamp()).unwrap();
        sqlx::query!(r#"
        update events.events
        set datetime = $1
        where message_id = $2
        "#, datetime, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_role_max(&self, message_id: MessageId, role: EventRole, max: usize) -> Result<()> {
        sqlx::query!(r#"
        update events.player_roles
        set max = $1
        where message_id = $2 and role = $3
        "#, max as i16, message_id.get() as i64, role as EventRole)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn signup_player(&self, message_id: MessageId, role: EventRole, player: Player) -> Result<()> {
        sqlx::query!(r#"
            delete from events.players
            where message_id = $1 and user_id = $2
            "#, message_id.get() as i64, player.id.get() as i64)
            .execute(&self.pool).await?;

        sqlx::query!(r#"
            insert into events.players(message_id,role,user_id,name,class)
            values($1,$2,$3,$4,$5)
            "#, message_id.get() as i64, role as EventRole, player.id.get() as i64, player.name, player.class as Option<PlayerClass>)
            .execute(&self.pool).await?;

        sqlx::query!(r#"
            delete from events.flex_roles
            where message_id = $1 and user_id = $2
            "#, message_id.get() as i64, player.id.get() as i64)
            .execute(&self.pool).await?;
        for role in player.flex {
            sqlx::query!(r#"
            insert into events.flex_roles(message_id,role,user_id)
            values($1,$2,$3)
            "#, message_id.get() as i64, role as EventRole, player.id.get() as i64)
                .execute(&self.pool).await?;
        }

        Ok(())
    }

    pub async fn remove_event(&self, message_id: MessageId) -> Result<()> {
        sqlx::query!(r#"
            delete from events.events
            where message_id = $1
            "#, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_id(&self, old_message_id: MessageId, new_message_id: MessageId) -> Result<()> {
        sqlx::query!(r#"
        insert into events.events (title, message_id, kind, scope, description, datetime, duration, leader, scheduled_event)
        select title, $1, kind, scope, description, datetime, duration, leader, scheduled_event
        from events.events
        where message_id = $2"#, new_message_id.get() as i64, old_message_id.get() as i64)
            .fetch_one(&self.pool).await?;

        sqlx::query!(r#"
        insert into events.player_roles (message_id, role, max)
        select $1, role, max
        from events.player_roles
        where message_id = $2"#, new_message_id.get() as i64, old_message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        sqlx::query!(r#"
        insert into events.players (message_id, role, user_id, name, class)
        select $1, role, user_id, name, class
        from events.players
        where message_id = $2"#, new_message_id.get() as i64, old_message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        sqlx::query!(r#"
        insert into events.flex_roles (message_id, role, user_id)
        select $1, role, user_id
        from events.flex_roles
        where message_id = $2"#, new_message_id.get() as i64, old_message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        self.remove_event(old_message_id).await
    }
}

struct DbEvent {
    title: String,
    kind: EventKind,
    scope: EventScopes,
    description: String,
    datetime: Option<OffsetDateTime>,
    duration: String,
    leader: i64,
    scheduled_event: Option<i64>
}

struct DbPlayerRole {
    message_id: i64,
    role: EventRole,
    max: Option<i16>
}

struct DbPlayer {
    message_id: i64,
    role: EventRole,
    user_id: i64,
    name: String,
    class: Option<PlayerClass>
}

struct DbFlexRole {
    message_id: i64,
    role: EventRole
}

impl Into<Event> for DbEvent {
    fn into(self) -> Event {
        Event {
            title: self.title,
            description: self.description,
            scope: self.scope,
            kind: self.kind,
            datetime: self.datetime.map(|dt| DateTime::<Utc>::from_timestamp_millis(dt.unix_timestamp())).flatten(),
            leader: UserId::new(self.leader as u64),
            roles: vec![],
            duration: DurationString::from_string(self.duration).unwrap(),
            scheduled_event: self.scheduled_event.map(|s| ScheduledEventId::new(s as u64))
        }
    }
}

impl Into<PlayersInRole> for DbPlayerRole {
    fn into(self) -> PlayersInRole {
        PlayersInRole {
            role: self.role,
            max: self.max.map(|m| m as usize),
            players: vec![]
        }
    }
}

impl Into<Player> for &DbPlayer {
    fn into(self) -> Player {
        Player {
            id: UserId::new(self.user_id as u64),
            name: self.name.to_string(),
            class: self.class.clone(),
            flex: vec![]
        }
    }
}