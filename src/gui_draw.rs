use crate::db::{
    filter_by_format, filter_by_rating, filter_by_release, filter_by_title,
    get_sub_reviews_from_reviewID,
};
use crate::gui_events::{
    get_all, get_all_movie_details, get_pool, get_sub_review_list, handle_init, ToGui,
};

use crate::{actorlist_to_string, parse_movie_list, parse_result, string_to_shared_string};
use once_cell::sync::Lazy;
use slint;
use slint::spawn_local;
use slint::SharedString;
use sqlx::{pool, MySqlPool};
use std::sync::Mutex;
// Shared state
static POOL: Lazy<Mutex<Option<MySqlPool>>> = Lazy::new(|| Mutex::new(None));

slint::slint! {
    import { Button, ListView, ScrollView, GridBox, Slider, ComboBox, CheckBox, Switch, StandardTableView, TabWidget} from "std-widgets.slint";
    export component MainGui inherits Window{
        InitButtonVisible: true;
        AllOtherVisible: false;
        ResetVisible: false;
        MovieThumbnailPath: "https://www.google.com/url?sa=i&url=https%3A%2F%2Fwww.imdb.com%2Ftitle%2Ftt0111161%2F&psig=AOvVaw3";
        MovieDetailsVisible: false;
        BasicColor: #1a2646;
        DialogVisible: false;
        //size of the window
        width: 800px;
        height: 790px;
        title: "Movie Database";
        in property <[string]> MoiveList;
        in property <[string]> SubReviewList;
        in property <color> BasicColor;
        in property <bool> InitButtonVisible;
        in property <bool> AllOtherVisible;
        out property <bool> ResetVisible;
        in property <string> MovieTitleIN;
        in property <string> MovieThumbnailPath;
        in property <string> Format;
        in property <string> Description;
        in property <string> Cast;
        in property <string> Review;
        in property <int> ReleaseDate;
        out property <bool> DialogVisible;
        out property <string> Filter;
        out property <string> SearchTerm;
        callback eventOccured();
        out property <string> Event;
        out property <string> MovieTitleOUT;
        out property <int> ReleaseDateOUT;
        out property <string> FormatOUT;
        out property <string> DescriptionOUT;
        out property <string> CastOUT;
        out property <int> ReviewOUT;
        out property <string> SubReviewOUT;
        out property <bool> MovieDetailsVisible;
        ComboBox {
            height: 27px;
            width: 102px;
            visible: true;
            enabled: AllOtherVisible;
            model: ["Filter-by", "Movie-Name", "Release-Date", "Format", "Rating", "Actor-Name"];
            x: 5px;
            y: 5px;
            selected => {
                root.eventOccured();
                Event = "FilterSelected";
                Filter = self.current-value;
            }
        }
        Rectangle {
            x: 697px;
            y: 9px;
            width: 102px;
            height: 25px;
            background: #4e4a4a;
            visible: true;
            border-radius: 5px;
            Switch {
                y: 0px;
                x: 0px;
                width: 104px;
                height: 24px;
                checked: false;
                text: "Connect";
                z: 99;
                visible: InitButtonVisible;
                toggled => {
                    eventOccured();
                    Event = "InitButtonClicked";
                 }
            }
            Text {
                text: "Connected";
                x: parent.width/6;
                y: parent.height/6;
                width: 104px;
                height: 24px;
                font-size: 14px;
                visible: !InitButtonVisible;
            }
        }
        Button {
            text: "Search";
            x: 470px;
            y: 9px;
            visible: true;
            enabled: AllOtherVisible;
            height: 25px;
            clicked => {
                eventOccured();
                Event = "SearchButtonClicked";
                ResetVisible = true;
            }
        }
        Rectangle {
            visible: AllOtherVisible;
            x: 12px;
            y: 69px;
            width: 263px;
            height: 723px;
            border-radius: 5px;
            border-width: 1px;
            background: BasicColor;
            Text {
                text: "Movies";
                x: 95px;
                y: 0px;
                height: 41px;
                font-size: 20px;
            }
            ListView {
                height: 689px;
                x: 5px;
                y: 26px;
                width: 252px;
                for data in MoiveList: Button {
                    width: 250px; // specify the width of the button
                    height: 50px; // specify the height of the button
                    text: data;
                    clicked => {
                        eventOccured();
                        Event = "MovieSelected";
                        MovieTitleOUT = data;
                        MovieDetailsVisible = true;

                    }
                }
}
        }
        Rectangle {
            visible: true;
            width: 345px;
            height: 31px;
            border-radius: 5px;
            border-width: 1px;
            background: BasicColor;
            x: 120px;
            y: 5px;
            TextInput {
                x: 10px;
                y: 7px;
                width: 327px;
                height: 22px;
                text: "Search";
                visible: true;
                enabled: AllOtherVisible;
                edited => {
                    eventOccured();
                    Event = "SearchTermEntered";
                    SearchTerm = self.text;
                    }
                }
            }
        Rectangle {
            Text {
                text: "Title: "+MovieTitleIN;
                x: 10px;
                y: 5px;
                width: 228px;
                height: 27px;
                font-size: 20px;
            }
            visible: MovieDetailsVisible;
            border-radius: 5px;
            border-width: 1px;
            background: BasicColor;
            width: 250px;
            height: 40px;
            x: 285px;
            y: 110px;
        }
        Rectangle {
            Text {
                text: "Release Year: "+ReleaseDate;
                x: 10px;
                y: 5px;
                width: 236px;
                height: 32px;
                font-size: 20px;
            }
            visible: MovieDetailsVisible;
            border-radius: 5px;
            border-width: 1px;
            background: BasicColor;
            width: 250px;
            height: 40px;
            x: 285px;
            y: 200px;
        }
        Rectangle {
            Text {
                text: "Format: "+Format;
                x: 10px;
                y: 5px;
                width: 234px;
                height: 32px;
                font-size: 20px;
            }
            width: 250px;
            height: 40px;
            border-radius: 5px;
            border-width: 1px;
            background: BasicColor;
            x: 285px;
            y: 155px;
            visible: MovieDetailsVisible;
        }
        Rectangle {
            Text {
                text:"Description:\n"+ Description;
                x: 10px;
                y: 5px;
                width: 491px;
                height: 139px;
                font-size: 20px;
            }
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            visible: MovieDetailsVisible;
            background: BasicColor;
            width: 505px;
            height: 140px;
            x: 285px;
            y: 290px;
        }
        Rectangle {
            Text {
                text: "Cast:\n"+Cast;
                x: 10px;
                y: 5px;
                width: 487px;
                height: 129px;
                font-size: 20px;
            }
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            visible: MovieDetailsVisible;
            background: BasicColor;
            width: 505px;
            height: 140px;
            x: 285px;
            y: 435px;
        }
        Rectangle {
            border-radius: 5px;
            border-width: 1px;
            border-color: #000;
            visible: MovieDetailsVisible;
            background: BasicColor;
            x: 285px;
            y: 580px;
            width: 510px;
            height: 215px;
            Text {
                text: "Reviews";
                x: 10px;
                y: 5px;
                height: 25px;
                font-size: 20px;
            }
            ListView {
                enabled: true;
                x: 5px;
                y: 30px;
                width: 496px;
                height: 174px;
                viewport-width: 1000px;
                for data in SubReviewList: Text {
                    text: data;
                    font-size: 14px;
                }
            }


        }
        Button {
            height: 25px;
            width: 84px;
            text: "+Add Movie";
            visible: true;
            enabled: AllOtherVisible;
            x: 605px;
            y: 9px;
            clicked => {
                eventOccured();
                Event = "AddMovieClicked";
                DialogVisible = true;
            }
        }
        Button {
            height: 25px;
            text: "Reset";
            x: 540px;
            y: 9px;
            visible: ResetVisible;
            clicked => {
                eventOccured();
                Event = "ResetButtonClicked";
                if Filter != "Filter-by"{
                    ResetVisible = false;
                }
            }
        }
        Rectangle {
            x: 285px;
            y: 245px;
            width: 250px;
            height: 40px;
            border-radius: 5px;
            border-width: 1px;
            background: BasicColor;
            visible: MovieDetailsVisible;
            Text {
                text: "Score: " + Review;
                x: 10px;
                y: 5px;
                height: 29px;
                font-size: 20px;

            }
        }
        Rectangle {
            x: -18px;
            y: -7px;
            width: 829px;
            height: 51px;
            border-color: #000;
            border-radius: 5px;
            border-width: 1px;
            background: #474343;
            z: -99;
        }
        Dialog {
            visible: DialogVisible;
            Rectangle {
                background: #1a2646;
                border-radius: 15px;
                border-width: 1px;
                height: 500px;
                width: 500px;
                Rectangle {
                    x: 9px;
                    y: 11px;
                    width: 480px;
                    height: 480px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #191a1b;
                }
                Button {
                    text: "Cancel";
                    x: 410px;
                    y: 10px;
                    height: 25px;
                    width: 80px;
                    z:99;
                    clicked => {
                        self.visible = false;
                        DialogVisible = false;
                    }
                }
                Button {
                    text: "Submit";
                    x: 215px;
                    y: 465px;
                    height: 25px;
                    width: 80px;
                    z:99;
                    clicked => {
                        self.visible = true;
                        DialogVisible = false;
                        eventOccured();
                        Event = "SubmitButtonClicked";
                    }
                }
                Rectangle {
                    x: 20px;
                    y: 100px;
                    z: 99;
                    width: 460px;
                    height: 25px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #53575f;
                    Text {
                        text: "Movie Title: ";
                        width: 105px;
                        height: 32px;
                        x: 10px;
                        y: 0px;
                        font-size: 20px;
                    }
                    TextInput {
                        x: 155px;
                        y: 5px;
                        width: 500px;
                        height: 25px;
                        text: "Enter Movie Title";
                        visible: true;
                        enabled: true;
                        edited => {
                            eventOccured();
                            Event = "MovieTitleEntered";
                            MovieTitleOUT = self.text;
                        }
                    }

                }
                Rectangle {
                    x: 20px;
                    y: 150px;
                    z: 99;
                    width: 460px;
                    height: 25px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #53575f;
                    Text {
                        text: "Release Year: ";
                        width: 150px;
                        height: 32px;
                        x: 10px;
                        y: 0px;
                        font-size: 20px;
                    }
                    TextInput {
                        x: 155px;
                        y: 5px;
                        width: 500px;
                        height: 25px;
                        text: "Enter Release Date";
                        visible: true;
                        enabled: true;
                        edited => {
                            eventOccured();
                            Event = "ReleaseDateEntered";
                            ReleaseDateOUT = self.text.to-float();

                        }
                    }
                }
                Rectangle {
                    x: 20px;
                    y: 200px;
                    z: 99;
                    width: 460px;
                    height: 25px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #53575f;
                    Text {
                        text: "Format: ";
                        width: 150px;
                        height: 32px;
                        x: 10px;
                        y: 0px;
                        font-size: 20px;
                    }
                    TextInput {
                        x: 155px;
                        y: 5px;
                        width: 500px;
                        height: 25px;
                        text: "Enter Format";
                        visible: true;
                        enabled: true;
                        edited => {
                            eventOccured();
                            Event = "FormatEntered";
                            FormatOUT = self.text;
                        }
                    }
                }
                Rectangle {
                    x: 20px;
                    y: 250px;
                    z: 99;
                    width: 230px;
                    height: 210px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #53575f;
                    Text {
                        text: "Description: ";
                        width: 150px;
                        height: 200px;
                        x: 10px;
                        y: 5px;
                        font-size: 20px;
                    }
                    TextInput {
                        x: 10px;
                        y: 30px;
                        width: 230px;
                        height: 190px;
                        text: "Enter Description";
                        visible: true;
                        enabled: true;
                        edited => {
                            eventOccured();
                            Event = "DescriptionEntered";
                            DescriptionOUT = self.text;
                        }
                    }
                }
                Rectangle {
                    x: 260px;
                    y: 250px;
                    z: 99;
                    width: 220px;
                    height: 210px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #53575f;
                    Text {
                        text: "Cast: ";
                        width: 150px;
                        height: 200px;
                        x: 10px;
                        y: 5px;
                        font-size: 20px;
                    }
                    TextInput {
                        x: 10px;
                        y: 30px;
                        width: 230px;
                        height: 190px;
                        text: "Enter Cast";
                        visible: true;
                        enabled: true;
                        edited => {
                            eventOccured();
                            Event = "CastEntered";
                            CastOUT = self.text;
                        }
                    }
                }
                Rectangle {
                    x: 10px;
                    y: 10px;
                    width: 480px;
                    height: 25px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #5f6a81;
                }
                Text {
                    text: "Add Movie";
                    width: 500px;
                    height: 32px;
                    x: 200px;
                    y: 10px;
                    font-size: 20px;
            }
            }
        }
    }
}

pub async fn init() {
    let app = MainGui::new().unwrap();
    let pool = get_pool().await;
    gui_loop(app, pool.unwrap()).await;
}

async fn gui_loop(app: MainGui, pool: MySqlPool) {
    let weak_app = app.as_weak();
    let weak_pool = pool.clone();
    app.on_eventOccured(move || {
        let app = weak_app.upgrade().unwrap();
        let pool = weak_pool.clone();
        let _ = spawn_local(async move {
            let event = app.get_Event();
            match event.as_str() {
                "InitButtonClicked" => init_button_clicked(app, pool).await,
                "SearchButtonClicked" => {
                    let filter = &app.get_Filter();
                    let search_term = app.get_SearchTerm();
                    search_by_filters(filter.to_string(), search_term.to_string(), app, pool).await;
                }
                "MovieSelected" => {
                    print!("Movie selected");
                    let movie_title = app.get_MovieTitleOUT();
                    get_movie_details(app, movie_title, pool).await;
                }
                "ResetButtonClicked" => {
                    let movie_list = filter_by_title(&pool, "".to_string()).await;
                    let model = parse_movie_list(movie_list);
                    app.set_MoiveList(model);
                }
                "SubmitButtonClicked" => {
                    let movie_title = app.get_MovieTitleOUT();
                    let release_date = app.get_ReleaseDateOUT();
                    let format = app.get_FormatOUT();
                    let description = app.get_DescriptionOUT();
                    let cast = app.get_CastOUT();
                    println!(
                        "Submit Button Clicked, Adding Movie: {} {} {} {} {}",
                        movie_title, release_date, format, description, cast
                    );
                }
                _ => {}
            }
        });
    });
    app.run().unwrap();
}

async fn init_button_clicked(app: MainGui, pool: MySqlPool) {
    app.set_InitButtonVisible(false);
    app.set_AllOtherVisible(true);
    populate_movie_list(app, pool).await;
}

async fn search_by_filters(filter: String, search_term: String, app: MainGui, pool: MySqlPool) {
    match filter.as_str() {
        "Movie-Name" => {
            let movie_list = filter_by_title(&pool, search_term).await;
            let model = parse_movie_list(movie_list);
            app.set_MoiveList(model);
        }
        "Release-Date" => {
            let search_term_int = search_term.parse::<i32>().unwrap(); // Convert search_term to i32
            let movie_list = filter_by_release(&pool, search_term_int).await;
            let model = parse_movie_list(movie_list);
            app.set_MoiveList(model);
        }
        "Format" => {
            let movie_list = filter_by_format(&pool, search_term).await;
            let model = parse_movie_list(movie_list);
            app.set_MoiveList(model);
        }
        "Rating" => {
            let search_term_int = search_term.parse::<i32>().unwrap(); // Convert search_term to i32
            let movie_list = filter_by_rating(&pool, search_term_int).await;
            let model = parse_movie_list(movie_list);
            app.set_MoiveList(model);
        }
        "Actor-Name" => {
            println!("Searching by actor name, {}", search_term);
        }
        _ => {
            println!("Invalid filter, {}", filter);
        }
    }
}

pub async fn get_movie_details(app: MainGui, movie_title: SharedString, pool: MySqlPool) {
    println!("Getting Details");
    //get movie detai
    let result = get_all_movie_details(&pool, movie_title.to_string()).await;
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
    app.set_ReleaseDate(result.MovieData[0].releaseDate.unwrap());
    println!("{}", actorlist_to_string(result.ActorData.clone()));
    app.set_Review(string_to_shared_string(
        result.ReviewData[0].aggregate.clone().unwrap().to_string(),
    ));
    populate_sub_review_list(app, pool, result.ReviewData[0].reviewID.unwrap()).await;
}

async fn populate_movie_list(app: MainGui, pool: MySqlPool) {
    let result = get_all(&pool).await;
    let model = parse_result(result);
    app.set_MoiveList(model);
}

async fn populate_sub_review_list(app: MainGui, pool: MySqlPool, review_id: i32) {
    let result = get_sub_review_list(&pool, review_id).await;
    let model = parse_result(result);
    print!("SubReviewList: {:?}", model);
    app.set_SubReviewList(model);
}
