use serenity::all::UserId;
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use crate::sets::request::GearRequest;
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Store {
    pool: PgPool
}

struct DbOrder {
    id: i32,
    kind: CraftingKind,
    owner: i64,
    crafter: Option<i64>,
    serialized_order: String,
    completed: bool,
    created_at: OffsetDateTime
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "events.kind", rename_all = "lowercase")]
enum CraftingKind {
    Set, Enchant, Consumables, Research
}

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Store {pool}
    }

    pub async fn create_set_order(&self, order: &GearRequest, for_user: UserId) -> Result<i32> {
        let row = sqlx::query!(r#"
        insert into crafting.orders(kind,owner,serialized_order)
        values($1,$2,$3) returning id
        "#, CraftingKind::Set as CraftingKind, for_user.get() as i64, serde_json::to_string(order)?)
            .fetch_one(&self.pool).await?;

        Ok(row.id)
    }
}