use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
   
#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    #[serde(alias = "originalTitle")]
    pub original_title: String,
    pub sypnosis: String,
    pub image: String,
    pub year: i16,
    pub duration: i16,
    #[serde(alias = "durationType")]
    pub duration_type: String,
    pub genres: Vec<String>,
    pub languages: Vec<String>,
    #[serde(alias = "resolutionWidth")]
    pub resolution_width: i16,
    #[serde(alias = "resolutionHeight")]
    pub resolution_height: i16,
    pub size: f32,
    #[serde(alias = "sizeType")]
    pub size_type: String,
    pub format: String
}