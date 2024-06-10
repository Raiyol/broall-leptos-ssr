use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::use_local_storage;
use leptos_use::{use_cookie_with_options, UseCookieOptions};
use leptos_use::utils::FromToStringCodec;
use thaw::{Theme, ThemeProvider};

use crate::components::header::HeaderComponent;
use crate::containers::chapter::ChapterPage;
use crate::containers::home::HomePage;
use crate::containers::not_found::NotFoundPage;
use crate::containers::novel::NovelPage;
use crate::containers::novels::NovelsPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (night_mode, set_night_mode) = use_cookie_with_options::<bool, FromToStringCodec>("night", UseCookieOptions::default().max_age(2592000_000_i64)); // 30 days
    let (night_mode_local, set_night_mode_local, _delete_night_mode_local) = use_local_storage::<bool, FromToStringCodec>("night");
    let theme = create_rw_signal(Theme::light());
    theme.update(|mut theme| {
        init_primary(&mut theme);
    });

    let on_toggle_theme = move |_| {
        set_night_mode_local.update(|val| *val = !*val);
    };

    create_effect(move |_| {
        set_night_mode.set(Some(night_mode_local.get()));
        let mut mode = if night_mode_local.get() { Theme::dark() } else { Theme::light() };
        init_primary(&mut mode);
        theme.set(mode);
    });

    fn init_primary(theme: &mut Theme) {
        theme.common.color_primary = "#6202EE".to_string();
        theme.common.color_primary_hover = "#BAA1D5".to_string();
        theme.common.color_primary_active = "#BAA1D5".to_string();
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/broall-leptos-ssr.css"/>

        // sets the document title
        <Title formatter=|text| format!("{text} - Broall")/>

        // Tooltip bundles
        <Script src="https://unpkg.com/@popperjs/core@2"/>
        <Script src="https://unpkg.com/tippy.js@6"/>

        {move || {
            let mode = if night_mode.get().map_or(false, |val| val) { "dark-mode" } else { "light-mode" };
            view! {<Body class=mode/>}
        }}

        // content for this welcome page
        <Router>
            <ThemeProvider theme>
                <HeaderComponent night_mode on_toggle_theme/>
                <main>
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/novels" view=NovelsPage/>
                        <Route path="/novels/:url" view=NovelPage/>
                        <Route path="/novels/:url/chapters/:number" view=ChapterPage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </ThemeProvider>
        </Router>
    }
}

/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <NotFoundPage /> }
}
