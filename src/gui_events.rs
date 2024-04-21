use std::result;

use sqlx::{pool, MySqlPool};

use crate::db::establish_connection;
use crate::db::filter_by_title;
use crate::db::get_all as get_all_records;
use crate::db::get_cast_from_movieID;
use crate::db::get_reviews_from_movieID;
use crate::record;
use crate::record::MicroReview;

#[derive(Clone)]
pub struct ToGui {
    pub MovieData: Vec<record::Record>,
    pub ActorData: Vec<record::CastMovieRecord>,
    pub ReviewData: Vec<record::Review>,
    pub MicroReviewData: Vec<record::MicroReview>,
    pub result: Vec<String>,
    pub pool: Option<MySqlPool>,
}

pub async fn get_pool() -> Option<MySqlPool> {
    let pool = establish_connection().await;
    match pool {
        Ok(pool) => Some(pool),
        Err(_) => None,
    }
}

pub async fn handle_init() -> ToGui {
    let mut result = ToGui {
        result: Vec::new(),
        pool: None,
        MovieData: Vec::new(),
        ActorData: Vec::new(),
        ReviewData: Vec::new(),
        MicroReviewData: Vec::new(),
    };

    match establish_connection().await {
        Ok(pool) => {
            result.pool = Some(pool);
            result.result.push("Connection established".to_string());
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    result
}

pub async fn get_all_movie_details(pool: &MySqlPool, movie_title: String) -> ToGui {
    let mut result = ToGui {
        result: Vec::new(),
        pool: None,
        MovieData: Vec::new(),
        ActorData: Vec::new(),
        ReviewData: Vec::new(),
        MicroReviewData: Vec::new(),
    };
    match filter_by_title(pool, movie_title).await {
        Ok(records) => {
            result.MovieData = records;
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    match get_reviews_from_movieID(pool, result.MovieData[0].movieId.unwrap()).await {
        Ok(records) => {
            result.ReviewData = records;
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    match get_cast_from_movieID(pool, result.MovieData[0].movieId.unwrap()).await {
        Ok(records) => {
            result.ActorData = records;
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    result
}

pub async fn get_all(pool: &MySqlPool) -> ToGui {
    let mut result = ToGui {
        result: Vec::new(),
        pool: None,
        MovieData: Vec::new(),
        ActorData: Vec::new(),
        ReviewData: Vec::new(),
        MicroReviewData: Vec::new(),
    };
    match get_all_records(pool).await {
        Ok(records) => {
            result.MovieData = records;
            for record in &result.MovieData {
                let title = record.title.as_ref().unwrap();
                result.result.push(format!("{}", title));
            }
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    result
}

pub async fn get_query() {}
