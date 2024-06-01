use leptos::{component, create_signal, IntoView, SignalUpdate, view};
use leptos_meta::Title;

/// Renders the home page of your application.
#[component]
pub fn NovelsPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Title text="Novels"/>
        <h1>"Novels"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
