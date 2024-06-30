use leptos::ServerFnError::WrappedServerError;
use leptos::{
    component, create_resource, create_signal, server, view, CollectView, IntoView, Params,
    ServerFnError, SignalGet, SignalGetUntracked, SignalWith, Transition,
};
use leptos_meta::Title;
use leptos_router::{use_params, Params, A};
use thaw::{Button, ButtonSize, ButtonVariant, Divider};

use crate::beans::novel_with_short_chapters::NovelWithShortChapters;
use crate::beans::pageable::Pageable;
use crate::beans::server_error_type::CustomError;
use crate::components::pagination::Pagination;
use crate::containers::not_found::NotFoundPage;

#[server]
pub async fn get_novel(url: String) -> Result<NovelWithShortChapters, ServerFnError<CustomError>> {
    crate::server::services::get_db_connexion_wrapper(|conn| {
        crate::server::services::novel_service::find_novel_by_url_with_chapters_info(conn, url)
    })
}

#[derive(Params, PartialEq)]
struct NovelParams {
    url: Option<String>,
}

#[component]
pub fn NovelPage() -> impl IntoView {
    let params = use_params::<NovelParams>();
    let url = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.url.clone())
                .unwrap_or_default()
                .unwrap_or_default()
        })
    };
    let novel_page = create_resource(url, |value| async move { get_novel(value).await });
    // Pagination for chapters
    let (page, set_page) = create_signal(Pageable::of(0, 1));

    view! {
        <div class="novel-page">
        <Transition fallback=move || view!{<p>"Loading..."</p>}>
        {
            move || novel_page.with(|result| match result {
                None => view! {""}.into_view(),
                Some(data) => {
                    match data {
                        Err(e) => {
                            match e {
                                WrappedServerError(CustomError::NotFound) =>  view! {<NotFoundPage/>}.into_view(),
                                _ => view! {""}.into_view()
                            }
                        },
                        Ok(novel) => {
                            let novel = novel.clone();
                            view! {
                                <Title text=novel.novel.name.clone()/>
                                <div class="titles">
                                    <h2 class="novel-title">{&novel.novel.name}</h2>
                                    <h3 class="og-novel-title">{&novel.novel.cn_name}</h3>
                                </div>
                                <div class="cover"><img src=format!("/assets/media/novel/{}", novel.novel.img.unwrap_or("unknown.jpg".to_string())) alt=&novel.novel.name/></div>
                                <ul class="genres">
                                    {novel.genres.into_iter().map(|genre| view! {<li><Button size=ButtonSize::Small variant=ButtonVariant::Outlined>{genre}</Button></li>}).collect_view()}
                                </ul>
                                <Divider/>
                                <div class="novel-summary">
                                    {novel.novel.summary.map(|summary| summary.split("\n")
                                        .map(|paragraphe| view! {<p>{paragraphe.to_string()}</p>})
                                        .collect_view()
                                    )}
                                </div>
                                <Divider/>
                                <div class="chapters">
                                    {
                                        move || view! {
                                            <Pagination count=page.get_untracked().get_total_page_count(novel.chapters.len()) />
                                            {
                                                let page_info = page.get();
                                                let start = (page_info.page_size * page_info.page_number) as usize;
                                                let end = ((page_info.page_number + 1) * page_info.page_size) as usize;
                                                view! {
                                                    <ul class="chapters-list">
                                                    {
                                                        novel.chapters[start..std::cmp::min(end, novel.chapters.len())].iter().map(|chapter| {
                                                            let chapter = chapter.clone();
                                                            view! {
                                                                <li><A class="link" href=format!("chapters/{}", chapter.number)><span>"#"{chapter.number}</span>{chapter.title_en.clone()}</A></li>
                                                            }
                                                        }).collect_view()
                                                    }
                                                    </ul>
                                                }
                                            }
                                            // <Pagination page count=page.get_untracked().get_total_page_count(novel.chapters.len()) set_page />
                                        }
                                    }
                                </div>
                            }.into_view()
                        }
                    }
                }
            })
        }
        </Transition>
        </div>
    }
}
