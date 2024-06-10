use leptos::{Callable, Callback, component, create_effect, create_rw_signal, IntoView, SignalGet, SignalSet, view};
use thaw::{Button, ButtonVariant, Select, SelectOption};

#[component]
pub fn ModalTextOptions(
    font_size_value: Option<i16>,
    line_height_value: Option<i16>,
    #[prop(into)]
    on_font_size_change: Callback<i16>,
    #[prop(into)]
    on_line_height_change: Callback<i16>,
) -> impl IntoView {
    let font_size_value = create_rw_signal(font_size_value);
    let line_height_value = create_rw_signal(line_height_value);
    let font_size_options = vec![10, 11, 12, 13, 14, 15, 16, 18, 20, 24, 32, 36, 40, 48, 64].into_iter().map(|val| SelectOption::new(val.to_string(), val)).collect::<Vec<SelectOption<i16>>>();
    let line_height_options = vec![100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200].into_iter().map(|val| SelectOption::new(format!("{}%", val), val)).collect::<Vec<SelectOption<i16>>>();

    create_effect(move |_| {
        on_font_size_change.call(font_size_value.get().unwrap_or(16));
        on_line_height_change.call(line_height_value.get().unwrap_or(160));
    });

    let on_reset = move |_| {
        font_size_value.set(Some(16));
        line_height_value.set(Some(160));
    };

    view! {
        <div class="modal-text-options">
            <div class="select">
                <span>"Font size:"</span>
                <Select options=font_size_options value=font_size_value/>
            </div>
            <div class="select">
                <span>"Line height:"</span>
                <Select options=line_height_options value=line_height_value/>
            </div>
            <Button variant=ButtonVariant::Outlined on_click=on_reset>"Reset"</Button>
        </div>
    }
}