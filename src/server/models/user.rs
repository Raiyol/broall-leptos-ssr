use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::server::schema::user;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = user)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub confirmed: i8,
    pub role: Option<String>,
    pub pfp: String,
    pub date: Option<NaiveDateTime>,
}
