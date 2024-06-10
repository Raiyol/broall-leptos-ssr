use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum CustomError {
    NotFound,
    Unknown,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "NotFound"),
            CustomError::Unknown => write!(f, "Unknown")
        }
    }
}

impl FromStr for CustomError {
    type Err = ();

    fn from_str(s: &str) -> leptos::error::Result<Self, Self::Err> {
        Ok(match s {
            "NotFound" => CustomError::NotFound,
            _ => CustomError::Unknown
        })
    }
}

impl std::error::Error for CustomError {}
