use leptos::{
    CollectView, component, create_resource, IntoView, server, ServerFnError, Transition, view,
};
use leptos_meta::Title;

use crate::beans::recent_chapter::RecentChapter;
use crate::beans::server_error_type::CustomError;

#[server]
pub async fn get_recent_novels() -> Result<Vec<RecentChapter>, ServerFnError<CustomError>> {
    crate::server::services::get_db_connexion_wrapper(|conn| {
        crate::server::services::chapter_service::get_recent_chapters(conn)
    })
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let home_page = create_resource(|| (), |_| async move { get_recent_novels().await });

    view! {
        <Title text="MTL of Chinese Webnovel"/>
        <h1>"Latest"</h1>
        <div class="home-page">
        <Transition fallback=move || view! {<p>Loading...</p>}>
            {move || home_page.and_then(|recent_chapters| view! {
                <ul>
                    {recent_chapters.iter().map(|chapter| {
                    view! {
                        <li>
                            <a class="link" href=format!("/novels/{}/chapters/{}", &chapter.url, chapter.chapter)>
                                <span class="chapter-number">"#"{chapter.chapter}</span>
                                <span class="novel-name">{&chapter.name}</span>
                            </a>
                            {chapter.date.and_then(|datetime| {
                                let diff = chrono::offset::Local::now().naive_utc() - datetime;
                                let diff_days = diff.num_days();
                                Some(view! {<span class="date">"("{diff_days} " day" {if diff_days > 1 {"s"} else {""}} " ago)"</span>})
                            })}
                        </li>
                    }}).collect_view()}
                </ul>
            })}
        </Transition>
        </div>
    }
}
