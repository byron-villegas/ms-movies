use serde::Serialize;

#[derive(Serialize)]
pub struct ValidationErrorDto {
    pub field: String,
    pub message: String
}