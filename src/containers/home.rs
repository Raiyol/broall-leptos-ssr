use leptos::{component, create_signal, server, view, IntoView, ServerFnError, SignalUpdate};
use leptos_meta::Title;

#[server(App)]
pub async fn get_recent_novels() -> Result<(), ServerFnError> {
    Ok(())
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Title text="MTL of Chinese Webnovel"/>
        <h1>"Latest"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
