use crate::gui_events::{get_all, get_all_movie_details, get_pool, handle_init, ToGui};
use crate::record::Record;
use crate::{actorlist_to_string, parse_result, string_to_shared_string};
use once_cell::sync::Lazy;
use slint;
use slint::spawn_local;
use slint::SharedString;
use sqlx::MySqlPool;
use std::sync::Mutex;
// Shared state
static POOL: Lazy<Mutex<Option<MySqlPool>>> = Lazy::new(|| Mutex::new(None));

slint::slint! {
    import { Button, ListView, ScrollView, GridBox, Slider, ComboBox} from "std-widgets.slint";
    export component MainGui inherits Window{
        InitButtonVisible: true;
        AllOtherVisible: false;
        title: "Movie Database";
        in property <[string]> MoiveList;
        in property <bool> InitButtonVisible;
        in property <bool> AllOtherVisible;
                root.eventOccured();
                Event = "SearchTermEdited";
            }
        }
        ComboBox {
            height: 30px;
            width: 106px;
            visible: AllOtherVisible;
            model: ["Filter-by", "Movie-Name", "Release-Date", "Format", "Description", "Actor-Name"];
            x: 1px;
            y: 0px;
            selected => {
                root.eventOccured();
                Event = "FilterSelected";
            }
        }
        Button {
            text: "Search";
            x: 452px;
            y: -1px;
            visible: AllOtherVisible;
            clicked => {
                eventOccured();
                Event = "SearchButtonClicked";
            }
        }
        Rectangle {
            visible: AllOtherVisible;
            x: 21px;
            y: 43px;
            width: 263px;
            height: 678px;
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            ListView {
                x: 21px;
                y: 34px;
                width: 236px;
                for data in MoiveList: Button {
                    width: 250px; // specify the width of the button
                    height: 50px; // specify the height of the button
                    text: data;
                    clicked => {
                        eventOccured();
                        Event = "MovieSelected";
                        MovieTitleOUT = data;

                }
            }
        }
    }

        Rectangle {
            visible: AllOtherVisible;
            width: 345px;
            height: 31px;
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            x: 106px;
            y: 0px;
        }
        Rectangle {
            Text {
                text: MovieTitleIN;
                x: 0px;
                y: 0px;
                height: 31px;
                font-size: 20px;
            }
            visible: AllOtherVisible;
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            width: 253px;
            height: 41px;
            x: 287px;
            y: 186px;
        }
        Rectangle {
            Text {
                text: Format;
                x: 0px;
                y: 0px;
                height: 31px;
                font-size: 20px;
            }
            width: 259px;
            height: 31px;
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            x: 289px;
            y: 241px;
            visible: AllOtherVisible;
        }
        Rectangle {
            Text {
                text: Description;
                x: 0px;
                y: 0px;
                height: 31px;
                font-size: 20px;
            }
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            visible: AllOtherVisible;
            width: 532px;
            height: 141px;
            x: 286px;
            y: 284px;
        }
        Rectangle {
            Text {
                text: Cast;
                x: 0px;
                y: 0px;
                height: 31px;
                font-size: 20px;
            }
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            visible: AllOtherVisible;
            width: 532px;
            height: 141px;
            x: 284px;
            y: 433px;
        }
        Rectangle {
            Text {
                text: Review;
                x: 0px;
                y: 0px;
                height: 31px;
                font-size: 20px;
            }
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            visible: AllOtherVisible;
            width: 532px;
            height: 141px;
            x: 283px;
            y: 578px;
        }
}

}

pub async fn init() {
    let app = MainGui::new().unwrap();
    gui_loop(app).await;
}

async fn gui_loop(app: MainGui) {
    let weakApp = app.as_weak();
    app.on_eventOccured(move || {
        let app = weakApp.upgrade().unwrap();
        let _ = spawn_local(async move {
            let event = app.get_Event();
            match event.as_str() {
                "InitButtonClicked" => init_button_clicked(app).await,
                "SearchButtonClicked" => {
                    print!("Search button clicked")
                }
                "FilterSelected" => {
                    print!("Filter selected")
                }
                "MovieSelected" => {
                    print!("Movie selected");
                    let movie_title = app.get_MovieTitleOUT();
                    get_movie_details(app, movie_title).await;
                }
                _ => {}
            }
        });
    });
    app.run().unwrap();
}

async fn init_button_clicked(app: MainGui) {
    let mut result = handle_init().await;
    app.set_InitButtonVisible(false);
    app.set_AllOtherVisible(true);

    println!("Result: {}", result.result.pop().unwrap());
    populate_movie_list(app, result).await;
}

pub async fn get_movie_details(app: MainGui, movie_title: SharedString) {
    println!("Getting Details");
    //get movie detai
    let pool_guard = get_pool().await;
    if let Some(ref actual_pool) = pool_guard {
        let result = get_all_movie_details(actual_pool, movie_title.to_string()).await;
        //display movie details
        app.set_MovieTitleIN(string_to_shared_string(
            result.MovieData[0].title.clone().unwrap(),
        ));
        app.set_Format(string_to_shared_string(
            result.MovieData[0].format.clone().unwrap(),
        ));
        app.set_Description(string_to_shared_string(
            result.MovieData[0].description.clone().unwrap(),
        ));
        app.set_Cast(string_to_shared_string(actorlist_to_string(
            result.ActorData.clone(),
        )));
    } else {
        println!("No pool found");
        return;
    }
}

async fn populate_movie_list(app: MainGui, mut result: ToGui) {
    result = get_all(&result.pool.unwrap()).await;
    let mut pool = POOL.lock().unwrap();
    *pool = result.pool.clone();
    let model = parse_result(result);
    app.set_MoiveList(model);
} //print all movies to console
