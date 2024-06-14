use leptos::ev::MouseEvent;
use leptos::{
    component, create_effect, create_local_resource, create_rw_signal, create_signal,
    event_target_value, server, view, CollectView, IntoView, ServerFnError, Signal, SignalGet,
    SignalSet, SignalUpdate, SignalWith,
};
use leptos_icons::Icon;
use leptos_use::signal_debounced;
use thaw::Button;

use crate::beans::novel_with_short_chapters::NovelResource;
use crate::beans::server_error_type::CustomError;

#[server]
pub async fn search_novel(
    input: Option<String>,
) -> Result<Vec<NovelResource>, ServerFnError<CustomError>> {
    crate::server::services::get_db_connexion_wrapper(|conn| {
        crate::server::services::novel_service::search_novel(conn, input)
    })
}

#[component]
pub fn HeaderComponent<F>(night_mode: Signal<Option<bool>>, on_toggle_theme: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let (show_menu, set_show_menu) = create_signal(false);
    let hide_menu = move |_| set_show_menu.set(false);
    let (show_search, set_show_search) = create_signal(false);
    let icon = create_rw_signal(Some(icondata::BsMoonStars));
    let (search, set_search) = create_signal("".to_string());
    let debounced_search: Signal<String> = signal_debounced(search, 500.0);
    let (focus_on_search, set_focus_on_search) = create_signal(false);
    let debounced_focus: Signal<bool> = signal_debounced(focus_on_search, 250.0);

    create_effect(move |_| {
        icon.update(|icon| {
            *icon = Some(if night_mode.get().map_or(false, |val| val) {
                icondata::BsSun
            } else {
                icondata::BsMoonStars
            })
        })
    });

    let search_result = create_local_resource(
        move || debounced_search.get(),
        |value| async move { search_novel(Some(value)).await },
    );

    view! {
        <header>
            <Button
                class="menu-button"
                icon=icondata::BiMenuRegular
                circle=true
                on_click=move |_| { set_show_menu.update(|val| *val = !*val) }
            />
            <a on:click=hide_menu class="title" href="/">
                "Broall"
            </a>
            <div class="search" class:show=move || show_search.get()>
                <div class="input">
                    <span><Icon icon=icondata::AiSearchOutlined/></span>
                    <input type="text" placeholder="Search..." value=move || search.get()
                        on:focus=move |_event| set_focus_on_search.set(true)
                        on:blur=move |_event| set_focus_on_search.set(false)
                        on:input=move |event| set_search.set(event_target_value(&event))/>
                </div>
                <div class="btn-search"><Button icon=icondata::ChCross on_click=move |_| set_show_search.set(false) circle=true/></div>
            </div>
            <div class="right">
                <Button class="open-search" icon=icondata::AiSearchOutlined on_click=move |_| set_show_search.set(true) circle=true/>
                <Button icon on_click=on_toggle_theme circle=true/>
            </div>
            <div class="search-results" class:focused=move || debounced_focus.get() && !search.get().is_empty() class=("show", move || show_search.get() && !search.get().is_empty())>
                <ul>
                    {move || search_result.with(|result| match result {
                        None => view! {""}.into_view(),
                        Some(Err(_e)) => view! {""}.into_view(),
                        Some(Ok(novels)) => view! {
                            <ul>
                                {novels.iter().map(|novel| {
                                    view! {
                                        <li>
                                            <a href=format!("/novels/{}", &novel.url)>
                                                <img src=format!("/assets/media/novel/{}", novel.clone().img.unwrap_or("unknown.jpg".to_string())) alt=&novel.name/>
                                                <div><span class="en">{&novel.name}</span><span class="cn">{&novel.cn_name}</span></div>
                                            </a>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_view(),
                    })}
                </ul>
            </div>
        </header>
        <nav
            class="nav-top"
            class:visible=move || show_menu.get()
        >
            <ul>
                <li>
                    <a on:click=hide_menu class="link" href="/">"Home"</a>
                </li>
                <li>
                    <a on:click=hide_menu class="link"  href="/novels">"Novels"</a>
                </li>
            </ul>
        </nav>
    }
}
