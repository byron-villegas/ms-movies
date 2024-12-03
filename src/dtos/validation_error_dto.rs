use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ValidationErrorDto {
    pub field: String,
    pub message: String
}