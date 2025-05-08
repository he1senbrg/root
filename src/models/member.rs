use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "sex_type")]
#[derive(Debug)]
pub enum Sex {
    M,
    F,
    Other,
}

#[derive(SimpleObject, FromRow)]
#[graphql(complex)]
#[derive(Debug)]
pub struct Member {
    pub member_id: i32,
    pub roll_no: String,
    pub name: String,
    pub email: String,
    pub sex: Sex,
    pub year: i32,
    pub hostel: String,
    pub mac_address: String,
    pub discord_id: String,
    pub group_id: i32,
    #[graphql(skip)] // Don't expose internal fields/meta-data
    pub created_at: NaiveDateTime,
}

#[derive(InputObject)]
pub struct CreateMemberInput {
    pub roll_no: String,
    pub name: String,
    pub email: String,
    pub sex: Sex,
    pub year: i32,
    pub hostel: String,
    pub mac_address: String,
    pub discord_id: String,
    pub group_id: i32,
}
