use diesel::prelude::*;
use crate::beans::novel_with_short_chapters::NovelResource;
use crate::server::models::chapter::ChapterInfo;

use crate::beans::novel_with_short_chapters::NovelWithShortChapters;
use crate::beans::page_response::PageResponse;
use crate::beans::pageable::Pageable;
use crate::server::models::genre::Genre;
use crate::server::models::novel::Novel;
use crate::server::models::novel_genre::NovelGenre;
use crate::server::schema::genre::name;
use crate::server::schema::*;

pub fn find_all_novel(
    conn: &mut MysqlConnection,
    pageable: Pageable,
) -> Result<PageResponse<NovelWithShortChapters>, diesel::result::Error> {
    let total: i64 = novel::table.count().get_result(conn)?;

    let nov = novel::table
        .limit(pageable.page_size)
        .offset(pageable.page_number * pageable.page_size)
        .select(Novel::as_select())
        .load::<Novel>(conn)?;

    let genres = NovelGenre::belonging_to(&nov)
        .inner_join(genre::table)
        .select((NovelGenre::as_select(), Genre::as_select()))
        .load::<(NovelGenre, Genre)>(conn)?;

    let grouped = genres
        .grouped_by(&nov)
        .into_iter()
        .zip(nov)
        .map(|(g, nov)| NovelWithShortChapters {
            novel: Novel::map_to_resource(nov),
            chapters: vec![],
            genres: g.into_iter().map(|(_, genre)| genre.name).collect(),
        })
        .collect::<Vec<NovelWithShortChapters>>();

    Ok(PageResponse::create(grouped, total, pageable))
}

pub fn find_novel_by_url_with_chapters_info(
    conn: &mut MysqlConnection,
    novel_url: String,
) -> Result<NovelWithShortChapters, diesel::result::Error> {
    let nov = novel::table
        .filter(novel::url.eq(novel_url))
        .select(Novel::as_select())
        .get_result(conn)?;

    let chapters = ChapterInfo::belonging_to(&nov)
        .select(ChapterInfo::as_select())
        .load(conn)?;

    let genres = NovelGenre::belonging_to(&nov)
        .inner_join(genre::table)
        .select(name)
        .load(conn)?;

    return Ok(NovelWithShortChapters {
        novel: Novel::map_to_resource(nov),
        chapters: chapters.into_iter().map(|chapter| ChapterInfo::map_to_resource(chapter)).collect(),
        genres,
    });
}

pub fn search_novel(
    conn: &mut MysqlConnection,
    search: Option<String>,
) -> Result<Vec<NovelResource>, diesel::result::Error> {
    let mut nov = novel::table.into_boxed();

    if let Some(value) = search {
        nov = nov
            .filter(novel::name.like(format!("%{}%", &value)))
            .or_filter(novel::cn_name.like(format!("%{}%", &value)));
    }

    nov.select(Novel::as_select()).load::<Novel>(conn).map(|novels| novels.into_iter().map(|nov| Novel::map_to_resource(nov)).collect())
}
