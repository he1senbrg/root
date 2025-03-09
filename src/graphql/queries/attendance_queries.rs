use std::sync::Arc;

use crate::models::attendance::{
    Attendance, AttendanceReport, AttendanceWithMember, DailyCount, MemberAttendanceSummary,
};
use async_graphql::{Context, Object, Result};
use chrono::NaiveDate;
use sqlx::PgPool;

#[derive(Default)]
pub struct AttendanceQueries;

#[Object]
impl AttendanceQueries {
    async fn attendance(&self, ctx: &Context<'_>, member_id: i32) -> Result<Vec<Attendance>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        Ok(
            sqlx::query_as::<_, Attendance>("SELECT * FROM Attendance WHERE member_id = $1")
                .bind(member_id)
                .fetch_all(pool.as_ref())
                .await?,
        )
    }

    async fn get_attendance_summary(
        &self,
        ctx: &Context<'_>,
        start_date: String,
        end_date: String,
    ) -> Result<AttendanceReport> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
            .map_err(|_| async_graphql::Error::new("Invalid start_date format. Use YYYY-MM-DD"))?;
        let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
            .map_err(|_| async_graphql::Error::new("Invalid end_date format. Use YYYY-MM-DD"))?;
        if start > end {
            return Err(async_graphql::Error::new(
                "startDate cannot be greater than endDate.",
            ));
        }

        let daily_count_result = sqlx::query!(
            r#"
           SELECT 
            attendace.date,
            COUNT(CASE WHEN attendace.is_present = true THEN attendace.member_id END) as total_present
            FROM Attendance attendace
            WHERE  attendace.date BETWEEN $1 AND $2
            GROUP BY attendace.date
            ORDER BY attendace.date
            "#,
            start,
            end
        )
        .fetch_all(pool.as_ref())
        .await;

        let daily_count_rows = daily_count_result?;

        let daily_count = daily_count_rows
            .into_iter()
            .map(|row| DailyCount {
                date: row.date.to_string(),
                count: row.total_present.unwrap_or(0),
            })
            .collect();

        let member_attendance_query = sqlx::query!(
            r#"
            SELECT member.member_id as "id!", member.name as "name!",
                COUNT(attendance.is_present)::int as "present_days!"
            FROM Member member
            LEFT JOIN Attendance attendance
                ON member.member_id = attendance.member_id
                AND attendance.is_present AND attendance.date >= CURRENT_DATE - INTERVAL '6 months'
            GROUP BY member.member_id, member.name
            ORDER BY member.member_id
            "#
        )
        .fetch_all(pool.as_ref())
        .await;

        let member_attendance = member_attendance_query?
            .into_iter()
            .map(|row| MemberAttendanceSummary {
                id: row.id,
                name: row.name,
                present_days: row.present_days as i64,
            })
            .collect();

        let max_days = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT date) FROM Attendance
        WHERE date >= CURRENT_DATE - INTERVAL '6 months' AND is_present",
        )
        .fetch_one(pool.as_ref())
        .await?;

        Ok(AttendanceReport {
            daily_count,
            member_attendance,
            max_days,
        })
    }

    async fn attendance_by_date(
        &self,
        ctx: &Context<'_>,
        date: NaiveDate,
    ) -> Result<Vec<AttendanceWithMember>> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool must be in context.");

        let records = sqlx::query_as::<_, AttendanceWithMember>(
            "SELECT att.attendance_id, att.member_id, att.date, att.is_present,
                    att.time_in, att.time_out, mem.name, mem.year
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
