use leptos::ServerFnError::WrappedServerError;
use leptos::{
    component, create_effect, create_resource, create_rw_signal, create_signal, server, view,
    CollectView, IntoView, MaybeSignal, Params, ServerFnError, SignalGet, SignalSet, SignalUpdate,
    SignalWith, Transition, WriteSignal,
};
use leptos_icons::Icon;
use leptos_meta::Title;
use leptos_router::{use_params, Params};
use leptos_use::core::StorageType;
use leptos_use::storage::{use_storage_with_options, UseStorageOptions};
use leptos_use::utils::FromToStringCodec;
use leptos_use::{use_cookie_with_options, UseCookieOptions};
use thaw::{Button, ButtonVariant, Modal};

use crate::beans::chapter_resource::ChapterResource;
use crate::beans::server_error_type::CustomError;
use crate::components::model_text_options::ModalTextOptions;
use crate::components::tooltip::Tooltip;
use crate::containers::not_found::NotFoundPage;

#[server]
pub async fn get_chapter(
    url: String,
    number: u32,
) -> Result<Option<ChapterResource>, ServerFnError<CustomError>> {
    crate::server::services::get_db_connexion_wrapper(|conn| {
        crate::server::services::chapter_service::get_chapter_by_id(conn, url, number)
    })
}

#[derive(Params, PartialEq)]
struct ChapterParams {
    url: Option<String>,
    number: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Word {
    cn: String,
    py: Option<String>,
    def: Option<String>,
}

#[component]
pub fn ChapterPage() -> impl IntoView {
    let params = use_params::<ChapterParams>();
    let params = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| {
                    (
                        params.url.clone().unwrap_or_default(),
                        params.number.clone().unwrap_or_default(),
                    )
                })
                .unwrap_or_default()
        })
    };
    let (show_en, set_show_en) = use_cookie_with_options::<bool, FromToStringCodec>(
        "en",
        UseCookieOptions::default().max_age(2_592_000_000_i64),
    ); // 30 days
    let (show_cn, set_show_cn) = use_cookie_with_options::<bool, FromToStringCodec>(
        "cn",
        UseCookieOptions::default().max_age(2_592_000_000_i64),
    ); // 30 days
    let (show_local_en, set_show_local_en, _delete_show_en) =
        use_storage_with_options::<bool, FromToStringCodec>(
            StorageType::Local,
            "en",
            UseStorageOptions::<bool, FromToStringCodec>::default().initial_value(true),
        );
    let (show_local_cn, set_show_local_cn, _delete_show_cn) =
        use_storage_with_options::<bool, FromToStringCodec>(
            StorageType::Local,
            "cn",
            UseStorageOptions::<bool, FromToStringCodec>::default().initial_value(false),
        );
    let en_btn_variant = MaybeSignal::derive(move || get_icon(show_en.get()));
    let cn_btn_variant = MaybeSignal::derive(move || get_icon(show_cn.get()));
    let show_options = create_rw_signal(false);

    let (font_size, set_font_size) = use_cookie_with_options::<i16, FromToStringCodec>(
        "font_size",
        UseCookieOptions::default().max_age(2_592_000_000_i64),
    ); // 30 days
    let (line_height, set_line_height) = use_cookie_with_options::<i16, FromToStringCodec>(
        "line_height",
        UseCookieOptions::default().max_age(2_592_000_000_i64),
    ); // 30 days
    let (font_size_local, set_font_size_local, _) =
        use_storage_with_options::<i16, FromToStringCodec>(
            StorageType::Local,
            "font_size",
            UseStorageOptions::<i16, FromToStringCodec>::default().initial_value(16),
        );
    let (line_height_local, set_line_height_local, _) =
        use_storage_with_options::<i16, FromToStringCodec>(
            StorageType::Local,
            "line_height",
            UseStorageOptions::<i16, FromToStringCodec>::default().initial_value(160),
        );

    create_effect(move |_| {
        set_font_size.set(Some(font_size_local.get()));
        set_line_height.set(Some(line_height_local.get()));
    });

    let update_lang: fn(WriteSignal<bool>, value: bool) = |local, value| {
        local.set(value);
    };

    create_effect(move |_| {
        set_show_en.set(Some(show_local_en.get()));
        set_show_cn.set(Some(show_local_cn.get()));
    });

    fn get_icon(show: Option<bool>) -> ButtonVariant {
        if show.map_or(false, |val| val) {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Outlined
        }
    }

    let chapter_page = create_resource(params, |value| async move {
        get_chapter(value.0, value.1).await
    });

    view! {
        <div class="chapter-page">
        <Transition fallback=move || view!{<p>"Loading..."</p>}>
        {
            move || chapter_page.with(|result| match result {
                None => view! {""}.into_view(),
                Some(data) => {
                    match data {
                        Err(e) => {
                            match e {
                                WrappedServerError(CustomError::NotFound) => view! {<NotFoundPage/>}.into_view(),
                                _ => view! {""}.into_view()
                            }
                        },
                        Ok(chapter_option) => match chapter_option {
                            None => view! {""}.into_view(),
                            Some(chapter) => {
                            let has_prev = chapter.number > chapter.first_chapter;
                            let has_next = chapter.number < chapter.last_chapter;
                            view! {
                                <Title text=format!("#{} {}", chapter.number, &chapter.name)/>
                                <div class="titles">
                                    <a class="link" href=format!("/novels/{}", params().0)>{&chapter.name}</a>
                                    <h2 id="title-en">"#"{chapter.number}": "{chapter.title_en.clone()}</h2>
                                </div>
                                <div class="actions">
                                    <div class="left">
                                        <a class="button button-outlined button-circle" href=format!("/novels/{}/chapters/{}", params().0, (params().1 as i32) - 1) style:visibility = move || if has_prev {"initial"} else {"hidden"}><Icon icon=icondata::AiCaretLeftFilled/></a>
                                        <a class="button button-primary button-circle" href=format!("/novels/{}", params().0)><Icon icon=icondata::AiOrderedListOutlined/></a>
                                        <a class="button button-outlined button-circle" href=format!("/novels/{}/chapters/{}", params().0, params().1 + 1) style:visibility = move || if has_next {"initial"} else {"hidden"}><Icon icon=icondata::AiCaretRightFilled/></a>
                                    </div>
                                    <Button icon=icondata::AiSettingOutlined on_click=move |_| show_options.set(true) circle=true />                            <Modal title="Options" width="300px" show=show_options>
                                        <ModalTextOptions font_size_value=font_size.get() line_height_value=line_height.get()
                                        on_font_size_change=move |v| set_font_size_local.set(v)  on_line_height_change=move |v| set_line_height_local.set(v) />
                                    </Modal>
                                    <div class="right">
                                        <Button variant=en_btn_variant on_click=move |_| update_lang(set_show_local_en, !show_local_en.get())>"EN"</Button>
                                        <Button variant=cn_btn_variant on_click=move |_| update_lang(set_show_local_cn, !show_local_cn.get())>"中文"</Button>
                                    </div>
                                </div>
                                <div class="content" style=("--font-size", move || font_size.get().map(|v| format!("{}px", v))) style=("--line-height", move || line_height.get().map(|v| format!("{}%", v)))>
                                    {chapter.content.clone().into_iter().enumerate().map(|(idx_line, line)| {
                                        let words: Vec<Word> = line.cn.into_iter().map(|word| {
                                            chapter.dict.get(&word).map(|w| w.clone())
                                                .map(|w| Word {py: Some(w.py), def: (!w.def.is_empty()).then_some(w.def), cn: word.clone()})
                                                .unwrap_or(Word {py: None, def: None, cn: word})
                                        }).collect();
                                        view! {
                                            {move || show_cn.get().and_then(|value| {
                                                if !value {
                                                    return None;
                                                }
                                                Some(view! {
                                                    <p class="cn">
                                                    {
                                                        words.clone().into_iter().enumerate().map(|(idx_word, word)| {
                                                            match word.def {
                                                                None => view! {{&word.cn}}.into_view(),
                                                                Some(_def) => {
                                                                    let (show_tooltip, set_show_tooltip) = create_signal(false);
                                                                    let _py = word.py.unwrap_or(String::from(""));
                                                                    #[cfg(not(feature = "ssr"))]
                                                                    let tooltip = Tooltip::new();
                                                                    #[cfg(not(feature = "ssr"))]
                                                                    let tooltip_ref = tooltip.clone();
                                                                    #[cfg(not(feature = "ssr"))]
                                                                    leptos::on_cleanup(move || {
                                                                        let _ = &tooltip_ref.destroy();
                                                                    });
                                                                    let toggle_tooltip = move |_| {
                                                                        set_show_tooltip.update(|val| *val = !*val);
                                                                        #[cfg(not(feature = "ssr"))]
                                                                        {
                                                                            if show_tooltip.get() {
                                                                                show(&tooltip, format!("t{}-{}", idx_line, idx_word), _py.clone(), _def.clone());
                                                                            } else {
                                                                                hide(&tooltip);
                                                                            }
                                                                        }
                                                                    };
                                                                    view! {
                                                                        <span class:box=move || show_tooltip.get() id=format!("t{}-{}", idx_line, idx_word) on:click=toggle_tooltip>{&word.cn}</span>
                                                                    }
                                                                }.into_view()
                                                            }
                                                        }).collect_view()
                                                    }
                                                    </p>
                                                })
                                            })}
                                            {move || show_en.get().and_then(|value| if value {Some(view! {<p class="en">{&line.en}</p>})} else {None})}
                                        }
                                    }.into_view() ).collect_view().into_view()}
                                </div>
                                <div class="bottom-nav">
                                    <a class="button button-outlined" href=format!("/novels/{}/chapters/{}", params().0, (params().1 as i32) - 1) style:visibility = move || if has_prev {"initial"} else {"hidden"}><Icon icon=icondata::AiCaretLeftFilled/> "Prev"</a>
                                    <a class="button button-primary" href=format!("/novels/{}", params().0)><Icon icon=icondata::AiOrderedListOutlined/> "ToC"</a>
                                    <a class="button button-outlined" href=format!("/novels/{}/chapters/{}", params().0, params().1 + 1)  style:visibility = move || if has_next {"initial"} else {"hidden"}>"Next" <Icon icon=icondata::AiCaretRightFilled/></a>
                                </div>
                            }}.into_view(),
                        }
                    }
                },
            })
        }
        </Transition>
        </div>
    }
}

pub fn show(tooltip: &Tooltip, id: String, py: String, def: String) {
    tooltip.create(
        &format!("#{}", id),
        format!(
            r#"<div class="tooltip dict">
        <div class="py">{}</div>
        <div class="def">{}</div>
    </div>"#,
            py, def
        ),
        true,
    );
    tooltip.show();
}

pub fn show_title(tooltip: &Tooltip, selector: &str, content: String) {
    tooltip.create(selector, content, false);
    tooltip.show();
}

pub fn hide(tooltip: &Tooltip) {
    tooltip.hide();
    tooltip.destroy();
}
