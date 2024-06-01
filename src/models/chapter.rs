use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::novel::Novel;
use crate::schema::schema::chapter;

#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Selectable, Associations, Debug, PartialEq,
)]
#[diesel(belongs_to(Novel, foreign_key = id_novel))]
#[diesel(table_name = chapter)]
pub struct Chapter {
    pub id: u32,
    pub id_novel: u32,
    pub number: u32,
    pub date: Option<NaiveDateTime>,
    pub title_en: Option<String>,
    pub title_cn: Option<String>,
    pub content: Option<String>,
    pub dict: Option<String>,
}

#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Selectable, Associations, Debug, PartialEq,
)]
#[diesel(belongs_to(Novel, foreign_key = id_novel))]
#[diesel(table_name = chapter)]
pub struct ChapterInfo {
    pub id: u32,
    pub id_novel: u32,
    pub number: u32,
    pub date: Option<NaiveDateTime>,
    pub title_en: Option<String>,
    pub title_cn: Option<String>,
}
