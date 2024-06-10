use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommentLightBean {
    pub id: u32,
    pub comment: String,
    pub date: Option<NaiveDateTime>,
    pub name: String,
    pub pfp: String,
}

impl CommentLightBean {
    pub fn map(
        id: u32,
        comment: String,
        date: Option<NaiveDateTime>,
        name: String,
        pfp: String,
    ) -> CommentLightBean {
        CommentLightBean {
            id,
            comment,
            date,
            name,
            pfp,
        }
    }
}
