use diesel::prelude::*;

use crate::beans::chapter_resource::ChapterResource;
use crate::server::models::chapter::Chapter;
use crate::server::models::novel::Novel;
use crate::server::schema::*;
use diesel::dsl::*;

pub fn get_chapter_by_id(
    conn: &mut MysqlConnection,
    novel_url: String,
    number: u32,
) -> Result<Option<ChapterResource>, diesel::result::Error> {
    let res = chapter::table
        .inner_join(novel::table)
        .filter(novel::url.eq(novel_url))
        .filter(chapter::number.eq(number))
        .select((Chapter::as_select(), Novel::as_select()))
        .first::<(Chapter, Novel)>(conn)
        .optional()?;

    let test: (Option<u32>, Option<u32>) = chapter::dsl::chapter
        .select((min(chapter::number), max(chapter::number)))
        .filter(chapter::id_novel.eq(1))
        .first::<(Option<u32>, Option<u32>)>(conn)
        .expect("Chapters/Novel should exist");

    Ok(res.map(|(chapter, novel)| {
        Chapter::map_to_resource(
            chapter,
            novel,
            test.0.expect("Novel should exist"),
            test.1.expect("Novel should exist"),
        )
    }))
}
