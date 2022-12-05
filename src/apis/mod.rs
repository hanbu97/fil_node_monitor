use axum::http::StatusCode;
use axum::Json;
use han_utils::res::Res;
use serde::{Deserialize, Serialize};

pub mod info;
pub mod inner;
pub mod subscribe;
pub mod history;