use leptos::{
    component, create_rw_signal, create_signal, view, IntoView, Signal, SignalGet, SignalSet,
    SignalUpdate, WriteSignal,
};
use thaw::Button;

#[component]
pub fn HeaderComponent(
    night_mode: Signal<bool>,
    set_night_mode: WriteSignal<bool>,
) -> impl IntoView {
    let (show_menu, set_show_menu) = create_signal(false);
    let icon = create_rw_signal(Some(icondata::BsSun));

    let toggle_theme = move |_| {
        set_night_mode.update(|val| *val = !*val);
        icon.update(|icon| {
            *icon = Some(if night_mode.get() {
                icondata::BsSun
            } else {
                icondata::BsMoonStars
            })
        })
    };

    let hide_menu = move |_| set_show_menu.set(false);

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
            <Button icon on_click=toggle_theme circle=true/>
        </header>
        <nav
            class="nav-top"
            style:visibility=move || if show_menu.get() { None } else { Some("hidden") }
        >
            <ul>
                <li>
                    <a on:click=hide_menu href="/novels">"Novels"</a>
                </li>
            </ul>
        </nav>
    }
}
