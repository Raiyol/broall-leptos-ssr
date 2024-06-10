use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::beans::novel_with_short_chapters::NovelResource;

use super::super::schema::*;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq)]
#[diesel(table_name = novel)]
pub struct Novel {
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

impl Novel {
    pub fn map_to_resource(self: Self) -> NovelResource {
        NovelResource {
            id: self.id,
            url: self.url,
            name: self.name,
            cn_name: self.cn_name,
            author: self.author,
            summary: self.summary,
            img: self.img,
            date: self.date,
            completed: self.completed,
        }
    }
}
