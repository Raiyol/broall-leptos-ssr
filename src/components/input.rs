use leptos::{component, IntoView, MaybeSignal, SignalGet, view};

#[component]
pub fn BroallInput(
    #[prop(optional, into)]
    label: MaybeSignal<String>,
    #[prop(optional, into)]
    id: MaybeSignal<String>,
) -> impl IntoView {
    let for_id = id.get();
    let label_value = label.get();
    let id_value = id.get();
    let has_label = move || label.get().len() > 0;

    view! {
        <div>
            {move || {
                let label_val = label_value.clone();
                let id_val = for_id.clone();
                has_label().then(|| {
                    if id_val.is_empty() {
                        view! {<label>{label_val}</label>}
                    } else {
                        view! {<label for=id_val>{label_val}</label>}
                    }
                })
            }}
            {move || {
                let id_val = id_value.clone();
                view! {<input id=id_val />}
            }}
        </div>
    }
}