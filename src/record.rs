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
    pub actor_age: Option<i32>,
    pub actor_role: Option<String>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct Review {
    pub reviewID: Option<i32>,
    pub aggregate: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub releaseDate: Option<i32>,
    pub sub_review_num: Option<i32>,
    pub movieId: Option<i32>,
}
#[derive(sqlx::FromRow, Clone)]
pub struct MicroReview {
    pub reviewID: Option<i32>,
    pub aggregate: Option<i32>,
    pub sub_review_num: Option<i32>,
    pub movieId: Option<i32>,
}
#[derive(sqlx::FromRow, Clone)]
pub struct SubReview {
    pub reviewID: Option<i32>,
    pub subreviewID: Option<i32>,
    pub sub_review_score: Option<i32>,
    pub sub_review_title: Option<String>,
    pub sub_review_desc: Option<String>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct MovieList {
    pub movieId: Option<i32>,
    pub title: Option<String>,
}

pub struct FromGui {
    pub actor_name: Vec<String>,
    pub actor_age: Vec<i32>,
    pub actor_role: Vec<String>,
    pub aggregate: i32,
    pub title: String,
    pub description: String,
    pub format: String,
    pub releaseDate: i32,
    pub sub_review_num: i32,
    pub sub_review_score: Vec<i32>,
    pub sub_review_title: Vec<String>,
    pub sub_review_desc: Vec<String>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct MovieId {
    pub movieId: Option<i32>,
}
pub struct CastId {
    pub castId: Option<i32>,
}
