#[derive(sqlx::FromRow, Clone)] // This attribute is used for automatic field mapping.
#[derive(Debug)]
pub struct Record {
    pub movieId: Option<i32>,
    pub title: Option<String>,
    pub releaseDate: Option<i32>,
    pub format: Option<String>,
    pub description: Option<String>,
}
#[derive(sqlx::FromRow, Clone)]
pub struct CastMovieRecord {
    pub movieId: Option<i32>,
    pub movie_title: Option<String>,
    pub actor_name: Option<String>,
    pub actor_age: Option<i32>, // Change this line
    pub actor_role: Option<String>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct Review {
    pub reviewID: Option<i32>,
    pub aggregate: Option<i32>,
    pub sub_review_num: Option<i32>,
    pub movieId: Option<i32>,
    pub title: Option<String>,
    pub releaseDate: Option<i32>,
    // pub genre: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
}
