use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub code: u16,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}
