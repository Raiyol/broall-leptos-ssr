use leptos::{component, IntoView, view};
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Not Found"/>
        <h1>"Not Found"</h1>
    }
}
