use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::beans::chapter_resource::ChapterResource;
use crate::beans::novel_with_short_chapters::ChapterInfoResource;

use crate::server::models::novel::Novel;
use crate::server::schema::chapter;

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

impl Chapter {
    pub fn map_to_resource(chapter: Self, novel: Novel, first_chapter: u32, last_chapter: u32) -> ChapterResource {
        ChapterResource {
            id: chapter.id,
            name: novel.name,
            number: chapter.number,
            date: chapter.date,
            title_en: chapter.title_en,
            title_cn: chapter.title_cn,
            content: chapter
                .content
                .map(|content| serde_json::from_str(&content).expect("wrong string format"))
                .unwrap_or(vec![]),
            dict: chapter
                .dict
                .map(|dict| serde_json::from_str(&dict).expect("wrong string format"))
                .unwrap_or(HashMap::new()),
            first_chapter,
            last_chapter,
        }
    }
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

impl ChapterInfo {
    pub fn map_to_resource(self: Self) -> ChapterInfoResource {
        ChapterInfoResource {
            id: self.id,
            id_novel: self.id_novel,
            number: self.number,
            date: self.date,
            title_en: self.title_en,
            title_cn: self.title_cn,
        }
    }
}
