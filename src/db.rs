use crate::record::CastId;
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
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).await
}
pub async fn get_max_review_id(pool: &MySqlPool) -> Result<i32, sqlx::Error> {
    let record = sqlx::query_scalar("SELECT MAX(reviewID) as reviewID FROM Review")
        .fetch_one(pool)
        .await?;
    Ok(record)
}

pub async fn get_max_sub_review_id(pool: &MySqlPool) -> Result<i32, sqlx::Error> {
    let record = sqlx::query_scalar("SELECT MAX(subreviewID) as subreviewID FROM Sub_Review")
        .fetch_one(pool)
        .await?;
    Ok(record)
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

pub async fn filter_by_actor(
    pool: &MySqlPool,
    name: String,
) -> Result<Vec<MovieList>, sqlx::Error> {
    let name = name.to_lowercase();
    let records: Vec<MovieList> = sqlx::query_as!(
        MovieList,
        "SELECT c.movieId, m.title AS title
        FROM CastMembers c
        JOIN Movie m ON c.movieId = m.movieId
        WHERE c.name LIKE ?",
        format!("%{}%", name),
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn remove_movie_by_id(pool: &MySqlPool, movie_id: i32) -> Result<(), sqlx::Error> {
    // First, delete related records from the CastMembers table
    sqlx::query("DELETE FROM CastMembers WHERE movieId = ?")
        .bind(movie_id)
        .execute(pool)
        .await?;

    //Get reviewID assiocated with movieID
    let row = sqlx::query("SELECT reviewID FROM Review WHERE movieId = ?")
        .bind(movie_id)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let review_id: i32 = row.get(0);
        // Delete related records from the Sub_Review table
        sqlx::query("DELETE FROM Sub_Review WHERE reviewID = ?")
            .bind(review_id)
            .execute(pool)
            .await?;
        sqlx::query("DELETE FROM Review WHERE movieId = ?")
            .bind(movie_id)
            .execute(pool)
            .await?;
    }

    // Finally, delete the movie record from the Movie table
    sqlx::query("DELETE FROM Movie WHERE movieId = ?")
        .bind(movie_id)
        .execute(pool)
        .await?;

    Ok(())
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
pub async fn get_max_movie_id(pool: &MySqlPool) -> Result<MovieId, sqlx::Error> {
    let record = sqlx::query_as!(MovieId, "SELECT MAX(movieId) as movieId FROM Movie")
        .fetch_one(pool)
        .await?;
    Ok(record)
}

pub async fn get_max_cast_id(pool: &MySqlPool) -> Result<CastId, sqlx::Error> {
    let record = sqlx::query_as!(CastId, "SELECT MAX(castID) as castId FROM CastMembers")
        .fetch_one(pool)
        .await?;
    Ok(record)
}

pub async fn add(query: String, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(&query).execute(pool).await?;
    Ok(())
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
