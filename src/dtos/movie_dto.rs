use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MovieDto {
    pub id: i64,
    pub title: String,
    pub originalTitle: String,
    pub sypnosis: String,
    pub image: String,
    pub year: i16,
    pub duration: i16,
    pub durationType: String,
    pub genres: Vec<String>,
    pub languages: Vec<String>,
    pub resolutionWidth: i16,
    pub resolutionHeight: i16,
    pub size: f32,
    pub sizeType: String,
    pub format: String
}