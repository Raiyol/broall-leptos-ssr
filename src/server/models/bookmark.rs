use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = bookmark)]
pub struct Bookmark {
    pub id_user: i32,
    pub id_novel: i32,
    pub chapter: i32
}
