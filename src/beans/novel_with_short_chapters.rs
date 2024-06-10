use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NovelWithShortChapters {
    #[serde(flatten)]
    pub novel: NovelResource,
    pub genres: Vec<String>,
    pub chapters: Vec<ChapterInfoResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NovelResource {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub cn_name: String,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub img: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub completed: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterInfoResource {
    pub id: u32,
    pub id_novel: u32,
    pub number: u32,
    pub date: Option<NaiveDateTime>,
    pub title_en: Option<String>,
    pub title_cn: Option<String>,
}
