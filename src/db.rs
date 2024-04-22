use crate::record::CastMovieRecord;
use crate::record::MicroReview;
use crate::record::MovieId;
use crate::record::MovieList;
use crate::record::Record;
use crate::record::Review;
use crate::record::SubReview;

use sqlx;

use sqlx::MySqlPool;
use sqlx::Row;

//Enum to contain Structs from SQL Query - Kinda clunky
pub enum QueryResults {
    Movies(Vec<Record>),
    Cast(Vec<CastMovieRecord>),
    Review(Vec<Review>),
    MicroReview(Vec<MicroReview>),
}

pub async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).await
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Record>, sqlx::Error> {
    // Example implementation, adjust the query as needed

    let records: Vec<Record> = sqlx::query_as!(Record, "SELECT * FROM Movie")
        .fetch_all(pool)
        .await?;
    Ok(records)
}

pub async fn get_cast_from_movieID(
    pool: &MySqlPool,
    movie_id: i32,
) -> Result<Vec<CastMovieRecord>, sqlx::Error> {
    let records: Vec<CastMovieRecord> = sqlx::query_as!(
        CastMovieRecord,
        "SELECT c.movieId, m.title AS movie_title, c.name AS actor_name, c.age AS actor_age, c.role AS actor_role
        FROM CastMembers c
        JOIN Movie m ON m.movieId = c.movieId
        WHERE c.movieId = ?",
        movie_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_reviews_from_movieID(
    pool: &MySqlPool,
    movie_id: i32,
) -> Result<Vec<Review>, sqlx::Error> {
    let records: Vec<Review> = sqlx::query_as!(
        Review,
        "SELECT * FROM Movie NATURAL JOIN Review WHERE movieId = ?",
        movie_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn filter_by_title(
    pool: &MySqlPool,
    title: String,
) -> Result<Vec<MovieList>, sqlx::Error> {
    let records: Vec<MovieList> = sqlx::query_as!(
        MovieList,
        "SELECT title, movieId FROM Movie WHERE title LIKE ?",
        format!("%{}%", title)
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

async fn filter_by_actor(pool: &MySqlPool, name: String) -> Result<Vec<MovieList>, sqlx::Error> {
    let records: Vec<MovieList> = sqlx::query_as!(
        MovieList,
        "SELECT title, m.movieId
        FROM CastMembers c
        JOIN Movie m ON m.movieId = c.movieId
        WHERE c.name = ?",
        name
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn filter_by_release(
    pool: &MySqlPool,
    release: i32,
) -> Result<Vec<MovieList>, sqlx::Error> {
    let records: Vec<MovieList> = sqlx::query_as!(
        MovieList,
        "SELECT title, movieId FROM Movie WHERE releaseDate = ?",
        release
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}
pub async fn filter_by_format(
    pool: &MySqlPool,
    format: String,
) -> Result<Vec<MovieList>, sqlx::Error> {
    let records: Vec<MovieList> = sqlx::query_as!(
        MovieList,
        "SELECT title, movieId FROM Movie WHERE format = ?",
        format
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn filter_by_rating(
    pool: &MySqlPool,
    rating: i32,
) -> Result<Vec<MovieList>, sqlx::Error> {
    let records: Vec<MovieList> = sqlx::query_as!(
        MovieList,
        "SELECT title, movieId FROM Review NATURAL JOIN Movie WHERE aggregate = ?",
        rating // Unwrap the Option<i32> or use a default value
    )
    .fetch_all(pool)
    .await?;

    // Extracting movieId from each tuple
    Ok(records)
}

pub async fn get_sub_reviews_from_reviewID(
    pool: &MySqlPool,
    review_id: i32,
) -> Result<Vec<SubReview>, sqlx::Error> {
    let records: Vec<SubReview> = sqlx::query_as!(
        SubReview,
        "SELECT * FROM Sub_Review WHERE reviewID = ?",
        review_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_movie_details_from_title(
    pool: &MySqlPool,
    title: String,
) -> Result<Vec<Record>, sqlx::Error> {
    let records: Vec<Record> =
        sqlx::query_as!(Record, "SELECT * FROM Movie WHERE title = ?", title)
            .fetch_all(pool)
            .await?;
    Ok(records)
}
