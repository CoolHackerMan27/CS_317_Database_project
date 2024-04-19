use cs_317_movie_project::gui_draw;
#[tokio::main]
async fn main() {
    gui_draw::init().await;
}

// #[tokio::main]
// async fn main() {
//     println!("Welcome!");
//     println!("Initializing databse, this may take some time....");
//     let pool = match handle_init().await {
//         Ok(pool) => {
//             println!("Initialization successful.");
//             pool
//         }
//         Err(e) => {
//             println!("Initialization failed: {}", e);
//             return; // Exit the function if initialization fails
//         }
//     };
//     handle_loop(pool).await;
// }
