use leptos::{
    component, create_resource, create_signal, server, view, CollectView, IntoView, ServerFnError,
    SignalGet, Transition,
};
use leptos_meta::Title;
use leptos_router::A;
use thaw::Divider;

use crate::beans::novel_with_short_chapters::NovelWithShortChapters;
use crate::beans::page_response::PageResponse;
use crate::beans::pageable::Pageable;
use crate::beans::server_error_type::CustomError;
use crate::components::pagination::Pagination;

#[server]
pub async fn get_novels(
    page: Pageable,
) -> Result<PageResponse<NovelWithShortChapters>, ServerFnError<CustomError>> {
    crate::server::services::get_db_connexion_wrapper(|conn| {
        crate::server::services::novel_service::find_all_novel(conn, page)
    })
}

#[component]
pub fn NovelsPage() -> impl IntoView {
    // Creates a reactive value to update pagination
    let (page, set_page) = create_signal(Pageable::of(0, 10));
    let novels_page = create_resource(
        move || page.get(),
        |value| async move { get_novels(value).await },
    );

    view! {
        <Title text="Novels"/>
        <h1>"Novels"</h1>
        <Transition fallback=move || view!{<p>"Loading..."</p>}>
            {move || novels_page.get().map(|result|
                result.map(|page_response|
                    view! {
                        <Pagination page page_total=page_response.page_total set_page />
                        <ul class="novels">
                        {
                            let novels_in_view = page_response.results.len();
                            page_response.results.into_iter().enumerate().map(|(idx, novel)| {
                            let name = novel.novel.name.clone();
                            let last = idx + 1 == novels_in_view;
                            view! {
                                <li class="novel">
                                    <A class="novel-title link" href=novel.novel.url.clone()>{name}</A>
                                    <A class="novel-img" href=novel.novel.url>
                                        <img src=format!("/assets/media/novel/{}", novel.novel.img.unwrap_or("unknown.jpg".to_string())) alt=&novel.novel.name/>
                                    </A>
                                    <div class="novel-summary">
                                        {novel.novel.summary.map(|summary| summary.split('\n')
                                            .map(|paragraphe| view! {<p>{paragraphe.to_string()}</p>})
                                            .collect_view()
                                        )}
                                    </div>
                                </li>
                                {if last {None} else {Some(view! {<Divider/>})}}
                            }}).collect_view()
                        }
                        </ul>
                        <Pagination page page_total=page_response.page_total set_page />
                    }
                )
            )}
        </Transition>
    }
}
