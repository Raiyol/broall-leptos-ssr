use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::FromToStringCodec;
use thaw::{Theme, ThemeProvider};

use crate::components::header::HeaderComponent;
use crate::containers::home::HomePage;
use crate::containers::novels::NovelsPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (night_mode, set_night_mode, _delete_night_mode) =
        use_local_storage::<bool, FromToStringCodec>("night");
    let theme = create_rw_signal(Theme::light());
    theme.update(|theme| {
        theme.common.color_primary = "#6202EE".to_string();
        theme.common.color_primary_hover = "#BAA1D5".to_string();
        theme.common.color_primary_active = "#BAA1D5".to_string();
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/broall-leptos-ssr.css"/>

        // sets the document title
        <Title formatter=|text| format!("{text} - Broall")/>

        // content for this welcome page
        <ThemeProvider theme>
            <Router>
                <HeaderComponent night_mode set_night_mode/>
                <main
                    style=(
                        "--background-color",
                        move || if night_mode.get() { "#1F1926" } else { "white" },
                    )

                    style=("--color", move || if night_mode.get() { "white" } else { "#1F1926" })
                >

                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/novels" view=NovelsPage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        </ThemeProvider>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
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

    view! { <h1>"Not Found"</h1> }
}
