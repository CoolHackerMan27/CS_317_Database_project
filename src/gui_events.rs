use std::string;

use sqlx::MySqlPool;

use crate::db::establish_connection;
use crate::db::filter_by_title;
use crate::db::get_all as get_all_records;
use crate::db::get_cast_from_movieID;
use crate::db::get_max_movie_id;
use crate::db::get_movie_details_from_title;
use crate::db::get_reviews_from_movieID;
use crate::db::get_sub_reviews_from_reviewID;
use crate::db::remove_movie_by_id;
use crate::record;
use crate::record::FromGui;

#[derive(Clone)]

pub struct ToGui {
    pub MovieData: Vec<record::Record>,
    pub ActorData: Vec<record::CastMovieRecord>,
    pub ReviewData: Vec<record::Review>,
    pub MicroReviewData: Vec<record::MicroReview>,
    pub MovieList: Vec<record::MovieList>,
    pub MovieId: Vec<record::MovieId>,
    pub SubReview: Vec<record::SubReview>,
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
        MovieList: Vec::new(),
        MovieId: Vec::new(),
        MicroReviewData: Vec::new(),
        SubReview: Vec::new(),
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
pub async fn delete_data(title: String, pool: &MySqlPool) {
    let title_id = filter_by_title(pool, title).await;
    // handle error and get i32 from movieID
    match title_id {
        Ok(movie_id) => {
            for i in 0..movie_id.len() {
                let movie_id = movie_id[i].movieId.unwrap();
                match remove_movie_by_id(&pool, movie_id).await {
                    Ok(_) => {
                        println!("Movie deleted successfully");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub async fn add_movie(movie: FromGui, pool: &MySqlPool) {
    //Generate a new movieId
    let movieId = get_max_movie_id(pool).await.unwrap().movieId.unwrap() + 1;
    let title = movie.title;
    let releaseDate = movie.releaseDate;
    let format = movie.format;
    let description = movie.description;
    let query = format!("INSERT INTO Movie (movieId, title, releaseDate, format, description) VALUES ({}, '{}', {}, '{}', '{}')", movieId, title, releaseDate, format, description);
    match crate::db::add(query, pool).await {
        Ok(_) => {
            println!("Movie added successfully");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    //Add Cast Members
    let mut castId = 0;
    for i in 0..movie.actor_name.len() {
        castId = crate::db::get_max_cast_id(pool)
            .await
            .unwrap()
            .castId
            .unwrap()
            + 1;
        let actor_name = movie.actor_name.get(i).unwrap();
        let actor_age = movie.actor_age.get(i).unwrap();
        let actor_role = movie.actor_role.get(i).unwrap();
        let query = format!("INSERT INTO CastMembers (castId, movieId, age, name, role, mis) VALUES ({}, {}, '{:?}', '{:?}', '{:?}', '{}')", castId, movieId, actor_age, actor_name, actor_role, "".to_string());
        match crate::db::add(query, pool).await {
            Ok(_) => {
                println!("Cast Member added successfully");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

pub async fn get_all_movie_details(pool: &MySqlPool, movie_title: String) -> ToGui {
    let mut result = ToGui {
        result: Vec::new(),
        pool: None,
        MovieData: Vec::new(),
        ActorData: Vec::new(),
        ReviewData: Vec::new(),
        MovieList: Vec::new(),
        MovieId: Vec::new(),
        MicroReviewData: Vec::new(),
        SubReview: Vec::new(),
    };
    match get_movie_details_from_title(pool, movie_title).await {
        Ok(records) => {
            result.MovieData = records;
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    match get_reviews_from_movieID(pool, result.MovieData.get(0).unwrap().movieId.unwrap()).await {
        Ok(records) => {
            result.ReviewData = records;
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    match get_cast_from_movieID(pool, result.MovieData.get(0).unwrap().movieId.unwrap()).await {
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
        MovieList: Vec::new(),
        MovieId: Vec::new(),
        MicroReviewData: Vec::new(),
        SubReview: Vec::new(),
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

pub async fn get_sub_review_list(pool: &MySqlPool, review_id: i32) -> ToGui {
    let mut result = ToGui {
        result: Vec::new(),
        pool: None,
        MovieData: Vec::new(),
        ActorData: Vec::new(),
        ReviewData: Vec::new(),
        MovieList: Vec::new(),
        MovieId: Vec::new(),
        MicroReviewData: Vec::new(),
        SubReview: Vec::new(),
    };
    match get_sub_reviews_from_reviewID(pool, review_id).await {
        Ok(records) => {
            result.SubReview = records;
            for record in &result.SubReview {
                let title = record.sub_review_title.as_ref().unwrap();
                result.result.push(format!("{}", title));
                let desc = record.sub_review_desc.as_ref().unwrap();
                result.result.push(format!("{}", desc));
                let score = record.sub_review_score.as_ref().unwrap();
                result.result.push(format!("{}", score));
            }
        }
        Err(e) => {
            result.result.push(format!("Error: {}", e));
        }
    }
    result
}
