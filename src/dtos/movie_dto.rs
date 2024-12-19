use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MovieDto {
    pub id: i64,
    pub title: String,
    #[serde(rename = "originalTitle")]
    pub original_title: String,
    pub sypnosis: String,
    pub image: String,
    pub year: i16,
    pub duration: i16,
    #[serde(rename = "durationType")]
    pub duration_type: String,
    pub genres: Vec<String>,
    pub languages: Vec<String>,
    #[serde(rename = "resolutionWidth")]
    pub resolution_width: i16,
    #[serde(rename = "resolutionHeight")]
    pub resolution_height: i16,
    pub size: f32,
    #[serde(rename = "sizeType")]
    pub size_type: String,
    pub format: String
}