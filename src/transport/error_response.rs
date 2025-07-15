use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}
