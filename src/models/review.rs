use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::schema::review;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(Novel, foreign_key = id_novel))]
#[diesel(table_name = review)]
pub struct Review {
    pub id: u32,
    pub id_novel: u32,
    pub id_user: u32,
    pub rate: Option<i32>,
    pub text: Option<String>,
    pub date: Option<NaiveDateTime>,
}
