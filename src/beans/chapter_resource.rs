use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChapterResource {
    pub id: u32,
    pub name: String,
    pub number: u32,
    pub date: Option<NaiveDateTime>,
    pub title_en: Option<String>,
    pub title_cn: Option<String>,
    pub content: Vec<ChapterContent>,
    pub dict: HashMap<String, Dictionary>,
    pub first_chapter: u32,
    pub last_chapter: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChapterContent {
    pub cn: Vec<String>,
    pub en: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Dictionary {
    pub def: String,
    pub py: String,
}
