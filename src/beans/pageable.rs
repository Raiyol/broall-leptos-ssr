use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pageable {
    #[validate(range(min = 0, message = "Page number must be positive"))]
    pub page_number: i64,
    #[validate(range(min = 1, max = 100, message = "Page size must be between 1 and 100"))]
    pub page_size: i64,
    pub sort: Option<String>,
    pub sort_order: Option<String>,
}

impl Pageable {
    pub fn new(page_number: i64, page_size: i64, sort: Option<String>, sort_order: Option<String>) -> Self {
        Self {
            page_number,
            page_size,
            sort,
            sort_order,
        }
    }

    pub fn of(page_number: i64, page_size: i64) -> Self {
        Self::new(page_number, page_size, None, None)
    }

    pub fn next(&mut self) {
        self.page_number += 1;
    }

    pub fn previous(&mut self) {
        self.page_number -= 1;
    }

    pub fn go_to(&mut self, page_number: i64) {
        self.page_number = page_number;
    }

    pub fn get_total_page_count(&self, total_count: usize) -> i64 {
        let total_count = total_count as i64;
        if self.page_size > 0 {
            (total_count + self.page_size - 1) / self.page_size
        } else {
            1
        }
    }
}
