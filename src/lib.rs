pub mod db;
pub mod gui_draw;
pub mod gui_events;
pub mod record;

use gui_events::ToGui;
use record::{MovieList, Record};
use slint::{Model, ModelRc, SharedString, VecModel};
use std::{
    fmt::Error,
    io::{self, Write},
    slice::SliceIndex,
};

pub fn string_to_shared_string(string: String) -> slint::SharedString {
    slint::SharedString::from(string)
}

pub fn model_rc_to_string(input: ModelRc<slint::SharedString>) -> String {
    let mut string = String::new();
    for item in input.iter() {
        string.push_str(&format!("{}\n", item));
    }
    string
}

pub fn actorlist_to_string(actorlist: Vec<record::CastMovieRecord>) -> String {
    let mut actor_string = String::new();
    for actor in actorlist.iter() {
        actor_string.push_str(&format!(
            "Name: {}, Age: {}, Role: {}\n",
            <std::option::Option<std::string::String> as Clone>::clone(&actor.actor_name).unwrap(), // Wow, this is a lot of boilerplate
            actor.actor_age.unwrap(),
            <std::option::Option<std::string::String> as Clone>::clone(&actor.actor_role).unwrap() //Truly terrible
        ));
    }
    actor_string
}
pub async fn get_user_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Flush the output buffer
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
pub async fn get_user_input_i32() -> std::result::Result<i32, sqlx::Error> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| sqlx::Error::Io(e))?;
    input
        .trim()
        .parse::<i32>()
        .map_err(|e| sqlx::Error::Configuration(e.into()))
}

pub fn parse_result(result: ToGui) -> ModelRc<slint::SharedString> {
    let vec = clean_result(result);
    let shared_string: Vec<slint::SharedString> = vec.into_iter().map(Into::into).collect();
    for movie in shared_string.iter() {
        println!("{}", movie);
    }
    let model = slint::ModelRc::new(VecModel::from(shared_string));
    return model;
}

pub fn parse_movie_list(
    movie_list: Result<Vec<MovieList>, sqlx::Error>,
) -> ModelRc<slint::SharedString> {
    if let record = movie_list.unwrap() {
        let mut vec = Vec::new();
        for movie in record.iter() {
            let movie_string = format!("{}", movie.title.as_ref().unwrap(),);
            vec.push(to_shared_string(movie_string));
        }
        slint::ModelRc::new(VecModel::from(vec))
    } else {
        println!("Query Error");
        //return error in model
        slint::ModelRc::new(VecModel::from(vec!["Query Error".into()]))
    }
}

fn to_shared_string(input: String) -> SharedString {
    input.into()
}

fn clean_result(result: ToGui) -> Vec<String> {
    let vec = result.result;
    //Record { movieId: Some(1), title: Some("Inception"), releaseDate: Some(2010), format: Some("Blu-ray"), description: Some("A mind-bending heist thriller directed by Christopher Nolan") }
    //remove the Record { and } from the string
    let mut clean_vec = Vec::new();
    for record in vec.iter() {
        let mut record = record.replace("Record {", "");
        record = record.replace("}", "");
        record = record.trim().to_string();
        record = record.replace("Some(", "");
        record = record.replace(")", "");
        clean_vec.push(record);
    }
    clean_vec
}
