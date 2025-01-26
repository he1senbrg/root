use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::models::status_update_streak::{StatusUpdateStreak as Streak, StreakInput};

#[derive(Default)]
pub struct StreakMutations;

#[Object]
impl StreakMutations {
    #[graphql(name = "incrementStreak")]
    async fn increment_streak(&self, ctx: &Context<'_>, input: StreakInput) -> Result<Streak> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let query = sqlx::query_as::<_, Streak>(
            "
        INSERT INTO StatusUpdateStreak VALUES ($1, 1, 1) 
        ON CONFLICT (member_id) DO UPDATE SET 
            current_streak = StatusUpdateStreak.current_streak + 1, 
            max_streak = GREATEST(StatusUpdateStreak.max_streak, StatusUpdateStreak.current_streak + 1)
        RETURNING *",
        )
        .bind(input.member_id);

        let updated_streak = query.fetch_one(pool.as_ref()).await?;

        Ok(updated_streak)
    }

    async fn reset_streak(&self, ctx: &Context<'_>, input: StreakInput) -> Result<Streak> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let query = sqlx::query_as::<_, Streak>(
            "
        INSERT INTO StatusUpdateStreak VALUES ($1, 0, 0) 
        ON CONFLICT (member_id) DO UPDATE 
            SET current_streak = 0
        RETURNING *",
        )
        .bind(input.member_id);

        let updated_streak = query.fetch_one(pool.as_ref()).await?;
        Ok(updated_streak)
    }
}
