
use bson::doc;
use mongodb::Database;

use crate::entities::movie::Movie;

pub async fn find_all(db: Database) -> Vec<Movie> {

    let mut cursor = db
    .collection::<Movie>("movies")
    .find(doc! {})
    .await
    .unwrap();

    let mut movies: Vec<Movie> = Vec::new();

    while cursor.advance().await.unwrap() {
        let movie = cursor.deserialize_current().unwrap();
        movies.push(movie);
    }
    
    return movies;
}