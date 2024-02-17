use chrono::{DateTime, Utc};
use duration_string::DurationString;
use serenity::all::{MessageId, ScheduledEventId, UserId};
use serenity::model::id::RoleId;
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use tracing::{info, instrument};
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

    #[instrument]
    pub async fn get_event(&self, message_id: MessageId) -> Result<Event> {
        info!("get event {}", message_id.get());
        let mut event: Event = sqlx::query_as!(DbEvent, r#"
        select
            title,
            kind as "kind!: EventKind",
            scope as "scope!: EventScopes",
            description, datetime, duration, leader, scheduled_event, notification_role
        from events.events
        where message_id = $1"#, message_id.get() as i64)
            .fetch_one(&self.pool).await?.into();

        let player_roles = sqlx::query_as!(DbPlayerRole, r#"
        select
            role as "role!: EventRole",
            max
        from events.player_roles
        where message_id = $1"#, message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        let players = sqlx::query_as!(DbPlayer, r#"
        select
            role as "role!: EventRole",
            user_id, name,
            class as "class!: Option<PlayerClass>"
        from events.players
        where message_id = $1"#, message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        let flex_roles: Vec<DbFlexRole> = sqlx::query_as!(DbFlexRole, r#"
        select
            user_id,
            role as "role!: EventRole"
        from events.flex_roles
        where message_id = $1"#, message_id.get() as i64)
            .fetch_all(&self.pool).await?;

        event.roles = player_roles.into_iter()
            .map(|pr| {
                let mut pr: PlayersInRole = pr.into();
                pr.players = players.iter()
                    .filter_map(|p| if p.role == pr.role {
                        let mut player: Player = p.into();
                        player.flex = flex_roles.iter()
                            .filter_map(|f| if f.user_id as u64 == player.id.get() {
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

    #[instrument]
    pub async fn create_event(&self, message_id: MessageId, event: &Event) -> Result<()> {
        info!("create event {}", message_id.get());
        sqlx::query!(r#"
        insert into events.events(message_id,kind,scope,title,description,duration,leader,datetime,scheduled_event,notification_role)
        values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
        "#, message_id.get() as i64,
            event.kind as EventKind,
            event.scope as EventScopes,
            event.title,
            event.description,
            event.duration.to_string(),
            event.leader.get() as i64,
            event.datetime.map(|dt| OffsetDateTime::from_unix_timestamp(dt.timestamp()).ok()).flatten(),
            event.scheduled_event.map(|e| e.get() as i64),
            event.notification_role.map(|e| e.get() as i64))
            .execute(&self.pool).await?;

        for pr in &event.roles {
            sqlx::query!(r#"
            insert into events.player_roles(message_id,role,max)
            values($1,$2,$3)
            "#, message_id.get() as i64, pr.role as EventRole, pr.max.map(|m| m as i16))
                .execute(&self.pool).await?;

            for player in &pr.players {
                self.signup_player(message_id, pr.role, &player).await?;
            }
        }

        Ok(())
    }

    #[instrument]
    pub async fn update_datetime(&self, message_id: MessageId, datetime: DateTime<Utc>) -> Result<()> {
        let datetime = OffsetDateTime::from_unix_timestamp(datetime.timestamp()).unwrap();
        info!("update datetime {datetime} for {}", message_id.get());
        sqlx::query!(r#"
        update events.events
        set datetime = $1
        where message_id = $2
        "#, datetime, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    #[instrument]
    pub async fn update_leader(&self, message_id: MessageId, leader: UserId) -> Result<()> {
        info!("update leader to {leader} for {}", message_id.get());
        sqlx::query!(r#"
        update events.events
        set leader = $1
        where message_id = $2
        "#, leader.get() as i64, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    #[instrument]
    pub async fn update_title(&self, message_id: MessageId, title: String) -> Result<()> {
        info!("update title to {title} for {}", message_id.get());
        sqlx::query!(r#"
        update events.events
        set title = $1
        where message_id = $2
        "#, title, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    #[instrument]
    pub async fn update_description(&self, message_id: MessageId, description: String) -> Result<()> {
        info!("update description to {description} for {}", message_id.get());
        sqlx::query!(r#"
        update events.events
        set description = $1
        where message_id = $2
        "#, description, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    #[instrument]
    pub async fn update_duration(&self, message_id: MessageId, duration: DurationString) -> Result<()> {
        info!("update duration to {duration} for {}", message_id.get());
        sqlx::query!(r#"
        update events.events
        set duration = $1
        where message_id = $2
        "#, duration.to_string(), message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
    }

    #[instrument]
    pub async fn signup_player(&self, message_id: MessageId, role: EventRole, player: &Player) -> Result<()> {
        info!("Delete players for {}", message_id.get());
        sqlx::query!(r#"
            delete from events.players
            where message_id = $1 and user_id = $2
            "#, message_id.get() as i64, player.id.get() as i64)
            .execute(&self.pool).await?;

        if let Some(class) = &player.class {
            info!("insert playersn in {role} with class {class} for {}", message_id.get());
            sqlx::query!(r#"
            insert into events.players(message_id,role,user_id,name,class)
            values($1,$2,$3,$4,$5)
            "#, message_id.get() as i64, role as EventRole, player.id.get() as i64, player.name, *class as PlayerClass)
                .execute(&self.pool).await?;
        } else {
            info!("insert players in {role} for {}", message_id.get());
            sqlx::query!(r#"
            insert into events.players(message_id,role,user_id,name)
            values($1,$2,$3,$4)
            "#, message_id.get() as i64, role as EventRole, player.id.get() as i64, player.name)
                .execute(&self.pool).await?;
        }

        info!("Delete flex for {} in {}", player.name, message_id.get());
        sqlx::query!(r#"
            delete from events.flex_roles
            where message_id = $1 and user_id = $2
            "#, message_id.get() as i64, player.id.get() as i64)
            .execute(&self.pool).await?;
        for role in &player.flex {
            info!("insert flex {role} for {} in {}", player.name, message_id.get());
            sqlx::query!(r#"
            insert into events.flex_roles(message_id,role,user_id)
            values($1,$2,$3)
            "#, message_id.get() as i64, *role as EventRole, player.id.get() as i64)
                .execute(&self.pool).await?;
        }

        Ok(())
    }

    #[instrument]
    pub async fn remove_event(&self, message_id: MessageId) -> Result<()> {
        info!("Remove event {}", message_id.get());
        sqlx::query!(r#"
            delete from events.events
            where message_id = $1
            "#, message_id.get() as i64)
            .execute(&self.pool).await?;
        Ok(())
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
    scheduled_event: Option<i64>,
    notification_role: Option<i64>
}

struct DbPlayerRole {
    role: EventRole,
    max: Option<i16>
}

struct DbPlayer {
    role: EventRole,
    user_id: i64,
    name: String,
    class: Option<PlayerClass>
}

struct DbFlexRole {
    role: EventRole,
    user_id: i64
}

impl Into<Event> for DbEvent {
    fn into(self) -> Event {
        Event {
            title: self.title,
            description: self.description,
            scope: self.scope,
            kind: self.kind,
            datetime: self.datetime.map(|dt| DateTime::<Utc>::from_timestamp(dt.unix_timestamp(), 0)).flatten(),
            leader: UserId::new(self.leader as u64),
            roles: vec![],
            duration: DurationString::from_string(self.duration).unwrap(),
            scheduled_event: self.scheduled_event.map(|s| ScheduledEventId::new(s as u64)),
            notification_role: self.notification_role.map(|s| RoleId::new(s as u64)),
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