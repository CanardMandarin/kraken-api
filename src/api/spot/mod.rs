use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    result: T,
}

pub mod public;
pub mod authenticated;