use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::novel::Novel;
use crate::schema::schema::novel_genre;
use crate::models::genre::Genre;

#[derive(Identifiable, Selectable, Serialize, Deserialize, Queryable, Associations, Debug, Clone)]
#[diesel(belongs_to(Novel, foreign_key = id_novel))]
#[diesel(belongs_to(Genre, foreign_key = id_genre))]
#[diesel(table_name = novel_genre)]
#[diesel(primary_key(id_novel, id_genre))]
pub struct NovelGenre {
    pub id_novel: u32,
    pub id_genre: u32,
}
