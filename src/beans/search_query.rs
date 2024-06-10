use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub search: Option<String>,
}
