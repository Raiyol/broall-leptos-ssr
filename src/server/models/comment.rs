use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::schema::comment;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq)]
#[diesel(table_name = comment)]
#[diesel(belongs_to(Chapter, foreign_key = id_chapter))]
#[diesel(belongs_to(User, foreign_key = id_user))]
pub struct Comment {
    pub id: u32,
    pub id_chapter: u32,
    pub id_user: u32,
    pub text: String,
    pub date: Option<NaiveDateTime>,
}
