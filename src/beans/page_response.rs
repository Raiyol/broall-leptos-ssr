use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::pageable::Pageable;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> where T: Debug {
    pub results: Vec<T>,
    pub total: i64,
    pub page_total: i64,
    pub has_next: bool,
    pub has_previous: bool,
}

impl<T> PageResponse<T> where T: Debug {
    pub fn create(content: Vec<T>, total: i64, pageable: Pageable) -> PageResponse<T> {
        let page_total = if pageable.page_size > 0 {
            (total + pageable.page_size - 1) / pageable.page_size
        } else {
            1
        };
        PageResponse {
            results: content,
            total,
            page_total,
            has_next: pageable.page_number < page_total - 1,
            has_previous: pageable.page_number > 0,
        }
    }
}
