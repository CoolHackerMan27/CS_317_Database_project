// use crate::db::establish_connection;
// use crate::db::QueryResults;
// use crate::gui_draw;
// use sqlx::mysql::MySqlPool;
// use std::io::{self, Write};

// pub async fn handle_loop(pool: MySqlPool) {
//     let mut answer: u16 = 0;
//     gui_draw::init();
//     while answer != 7 {
//         answer = display_menu();
//         handle_queries(&pool, answer).await;
//     }
// }

// fn handle_display(res: Vec<QueryResults>) {
//     for result in res {
//         match result {
//             QueryResults::Movies(records) => {
//                 println!("Movies:");
//                 for record in records {
//                     println!(
//                         "ID: {}, Title: {}, Release Date: {}, Format: {}, Description: {}",
//                         record.movieId.unwrap_or_default(),
//                         record.title.unwrap_or_default(),
//                         record.releaseDate.unwrap_or_default(),
//                         record.format.unwrap_or_default(),
//                         record.description.unwrap_or_default()
//                     );
//                 }
//             }
//             QueryResults::Cast(records) => {
//                 println!("Cast:");
//                 for record in records {
//                     println!("Movie ID: {}, Movie Title: {}, Actor Name: {}, Actor Age: {}, Actor Role: {}",
//                         record.movieId.unwrap_or_default(),
//                         record.movie_title.unwrap_or_default(),
//                         record.actor_name.unwrap_or_default(),
//                         record.actor_age.unwrap_or_default(),
//                         record.actor_role.unwrap_or_default()
//                     );
//                 }
//             }
//             QueryResults::Review(records) => {
//                 println!("Reviews:");
//                 for record in records {
//                     println!("Review ID: {}, Aggregate: {}, Sub Review Number: {}, Movie ID: {}, Title: {}, Release Date: {}, Format: {}, Description: {}",
//                         record.reviewID.unwrap_or_default(),
//                         record.aggregate.unwrap_or_default(),
//                         record.sub_review_num.unwrap_or_default(),
//                         record.movieId.unwrap_or_default(),
//                         record.title.unwrap_or_default(),
//                         record.releaseDate.unwrap_or_default(),
//                         record.format.unwrap_or_default(),
//                         record.description.unwrap_or_default()
//                     );
//                 }
//             }
//         }
//     }
// }

// fn display_menu() -> u16 {
//     let mut input = String::new(); // Define a mutable variable to store user input

//     println!("~~~~~~~Main-Menu~~~~~~~");
//     println!("Select your Option");
//     println!("1: List All Movies\n2: Filter by actor\n3: Filter by Title\n4: Filter by Reviews\n5: Filter By Format\n6: Add Movie\n7: Quit");
//     io::stdout().flush().unwrap(); // Make sure 'println!' doesn't get buffered.
//     io::stdin().read_line(&mut input).unwrap(); // Read user input into the 'input' variable

//     let answer: u16 = input.trim().parse().expect("Please type a number!"); // Parse the input as a u16
//     answer // Return the parsed number
// }

// // TO-DO Handle Gui
