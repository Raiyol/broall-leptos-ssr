use leptos::{component, create_signal, IntoView, server, ServerFnError, SignalUpdate, spawn_local, view};
use leptos_meta::Title;

use crate::beans::recent_chapter::RecentChapter;

#[server]
pub async fn get_recent_novels() -> Result<Vec<RecentChapter>, ServerFnError> {
    use diesel::r2d2::Error;
    use leptos::use_context;
    use diesel::prelude::*;
    use crate::server::models::chapter::ChapterInfo;
    use crate::server::models::novel::Novel;
    use crate::server::schema::*;

    let server_context = use_context::<crate::server::db::DbPool>();
    match server_context {
        Some(pool) => {
            let mut conn = pool.get().map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;
            let chapters_with_novel: Result<Vec<(ChapterInfo, Novel)>, diesel::result::Error> =
                chapter::table
                    .inner_join(novel::table)
                    .select((ChapterInfo::as_select(), Novel::as_select()))
                    .limit(20)
                    .order(chapter::date.desc())
                    .load::<(ChapterInfo, Novel)>(&mut *conn);

            Ok(
                chapters_with_novel.map(|op| {
                    op.into_iter()
                        .map(|(chapter_info, nov)| RecentChapter {
                            id: chapter_info.id,
                            url: nov.url.clone(),
                            name: nov.name.clone(),
                            chapter: chapter_info.number,
                            date: chapter_info.date,
                        })
                        .collect::<Vec<RecentChapter>>()
                }).map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?
            )
        }
        None => Err(ServerFnError::ServerError("Context error".to_string()))
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| {
        spawn_local(async {
            // get_recent_novels().await;
        });
        set_count.update(|count| *count += 1)
    };

    view! {
        <Title text="MTL of Chinese Webnovel"/>
        <h1>"Latest"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
