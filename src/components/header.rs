use leptos::{component, create_effect, create_local_resource, create_rw_signal, create_signal, event_target_value, IntoView, server, ServerFnError, Signal, SignalGet, SignalSet, SignalUpdate, view};
use leptos::ev::MouseEvent;
use leptos_icons::Icon;
use leptos_use::signal_debounced;
use thaw::Button;

use crate::beans::novel_with_short_chapters::NovelResource;
use crate::beans::server_error_type::CustomError;

#[server]
pub async fn search_novel(input: Option<String>) -> Result<Vec<NovelResource>, ServerFnError<CustomError>> {
    crate::server::services::get_db_connexion_wrapper(|conn| crate::server::services::novel_service::search_novel(conn, input))
}

#[component]
pub fn HeaderComponent<F>(
    night_mode: Signal<Option<bool>>,
    on_toggle_theme: F,
) -> impl IntoView
    where F: Fn(MouseEvent) + 'static {
    let (show_menu, set_show_menu) = create_signal(false);
    let (show_search, set_show_search) = create_signal(false);
    let icon = create_rw_signal(Some(icondata::BsMoonStars));
    let (search, set_search) = create_signal("".to_string());
    let debounced_search: Signal<String> = signal_debounced(search, 500.0);

    create_effect(move |_| {
        icon.update(|icon| *icon = Some(if night_mode.get().map_or(false, |val| val) { icondata::BsSun } else { icondata::BsMoonStars }))
    });

    let hide_menu = move |_| set_show_menu.set(false);

    let search_result = create_local_resource(move || debounced_search.get(), |value| async move {
        leptos::logging::log!("test");
        search_novel(Some(value)).await
    });

    view! {
        <header>
            <Button
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
                    <input type="text" value=move || search.get() on:input=move |event| set_search.set(event_target_value(&event))/>
                </div>
                <div class="btn-search"><Button icon=icondata::ChCross on_click=move |_| set_show_search.set(false) circle=true/></div>
            </div>
            <div class="right">
                <Button icon=icondata::AiSearchOutlined on_click=move |_| set_show_search.set(true) circle=true/>
                <Button icon on_click=on_toggle_theme circle=true/>
            </div>
        </header>
        <nav
            class="nav-top"
            style:visibility=move || if show_menu.get() { None } else { Some("hidden") }
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
