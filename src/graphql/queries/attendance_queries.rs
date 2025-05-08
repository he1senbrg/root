use std::sync::Arc;

use crate::models::attendance::{Attendance, AttendanceWithMember};
use async_graphql::{Context, Object, Result};
use chrono::NaiveDate;
use sqlx::PgPool;

#[derive(Default, Debug)]
pub struct AttendanceQueries;

#[Object]
impl AttendanceQueries {
    #[tracing::instrument(skip(ctx))]
    async fn attendance(&self, ctx: &Context<'_>, member_id: i32) -> Result<Vec<Attendance>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        tracing::info!("Fetching attendance for member ID: {}", member_id);

        Ok(
            sqlx::query_as::<_, Attendance>("SELECT * FROM Attendance WHERE member_id = $1")
                .bind(member_id)
                .fetch_all(pool.as_ref())
                .await?,
        )
    }

    async fn attendance_by_date(
        &self,
        ctx: &Context<'_>,
        date: NaiveDate,
    ) -> Result<Vec<AttendanceWithMember>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let records = sqlx::query_as::<_, AttendanceWithMember>(
            "SELECT att.attendance_id, att.member_id, att.date, att.is_present,
                    att.time_in, att.time_out, mem.name, mem.year, mem.group_id
             FROM Attendance att
             JOIN Member mem ON att.member_id = mem.member_id
             WHERE att.date = $1",
        )
        .bind(date)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(records)
    }
}
