use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentChapter {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub chapter: u32,
    pub date: Option<NaiveDateTime>,
}
