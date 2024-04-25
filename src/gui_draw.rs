use crate::db::{
    filter_by_actor, filter_by_format, filter_by_rating, filter_by_release, filter_by_title,
};
use crate::gui_events::{
    add_movie, delete_data, get_all, get_all_movie_details, get_pool, get_sub_review_list,
};

use crate::record::FromGui;
use crate::{
    actorlist_to_string, parse_movie_list, parse_result, string_to_shared_string, vec_str_to_model,
};
use once_cell::sync::Lazy;
use slint::spawn_local;
use slint::SharedString;
use slint::{self};
use sqlx::MySqlPool;

use std::sync::Mutex;

// Shared state
static CAST_LIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

slint::slint! {
    import { Button, ListView, ScrollView, GridBox, Slider, ComboBox, CheckBox, Switch, StandardTableView, TabWidget} from "std-widgets.slint";
    export component MainGui inherits Window{
        InitButtonVisible: true;
        AllOtherVisible: true;
        ResetVisible: false;
        MovieDetailsVisible: false;
        BasicColor: #1a2646;
        DialogVisible: false;
        NumOfCastMembers: 1;
        MovieList: ["Movie1", "Movie2", "Movie3"];
        //size of the window
        width: 800px;
        height: 790px;
        title: "Movie Database";
        in property <[string]> MovieList;
        in property <[string]> SubReviewList;
        in property <color> BasicColor;
        in property <bool> InitButtonVisible;
        in property <bool> AllOtherVisible;
        in property <string> MovieTitleIN;
        in property <string> MovieThumbnailPath;
        in property <string> Format;
        in property <string> Description;
        in property <string> Cast;
        in property <string> Review;
        in property <int> ReleaseDate;
        in property <[string]> CastListIN;
        out property <bool> ResetVisible;
        out property <bool> DialogVisible;
        out property <string> Filter;
        out property <string> SearchTerm;
        out property <string> Event;
        out property <string> MovieTitleOUT;
        out property <int> ReleaseDateOUT;
        out property <string> FormatOUT;
        out property <string> DescriptionOUT;
        out property <string> CastAgeOUT;
        out property <string> CastNameOUT;
        out property <string> CastRoleOUT;
        out property <int> ReviewOUT;
        out property <string> SubReviewOUT;
        out property <bool> MovieDetailsVisible;
        out property <int> NumOfCastMembers;
        callback eventOccured();
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
                for data in MovieList: Rectangle{
                    width: 250px; // specify the width of the Rectangle
                    height: 50px; // specify the height of the Rectangle
                    Button {
                        width: 250px; // specify the width of the button
                        height: 45px; // specify the height of the button
                        text: data;
                        clicked => {
                            eventOccured();
                            Event = "MovieSelected";
                            MovieTitleOUT = data;
                            MovieDetailsVisible = true;
                        }
                    }
                    Button{
                        width: 50px;
                        height: 45px;
                        text: "Delete";
                        x: 200px;
                        clicked => {
                            eventOccured();
                            Event = "DeleteMovieClicked";
                            MovieTitleOUT = data;
                        }
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
                        // Reset the Default values
                        MovieTitleOUT = "Enter Movie Title";
                        ReleaseDateOUT = 0;
                        FormatOUT = "Enter Format";
                        DescriptionOUT = "Enter Description";
                        ReviewOUT = 0;
                        SubReviewOUT = "Enter Sub Review";
                    }
                }
                Rectangle {
                    x: 20px;
                    y: 55px;
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
                    y: 100px;
                    z: 99;
                    width: 460px;
                    height: 25px;
                    border-radius: 5px;
                    border-width: 1px;
                    background: #53575f;
                    Text {
                        text: "Review Score: ";
                        width: 138px;
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
                        text: "Enter Review Score";
                        visible: true;
                        enabled: true;
                        edited => {
                            eventOccured();
                            Event = "ReviewScoreEntered";
                            ReviewOUT = self.text.to-float();
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
                    Button {
                        text: "Add Member";
                        x: 5px;
                        y: 80px;
                        height: 25px;
                        width: 100px;
                        z:99;
                        clicked => {
                            NumOfCastMembers += 1;
                            eventOccured();
                            Event = "AddMemberClicked";
                        }
                    }
                    Button {
                        text: "Remove Member";
                        x: 110px;
                        y: 80px;
                        height: 25px;
                        width: 100px;
                        z:99;
                        clicked => {
                            NumOfCastMembers -= 1;
                            eventOccured();
                            Event = "RemoveMemberClicked";
                        }
                    }
                    Text {
                        text: "Cast: ";
                        width: 69px;
                        height: 32px;
                        x: 83px;
                        y: 5px;
                        font-size: 20px;
                        z:99;
                    }
                    Rectangle {
                        background: #c8cdd6;
                        border-radius: 5px;
                        x: 5px;
                        y: 50px;
                        width: 60px;
                        height: 25px;
                        z:90;
                        Text {
                            text: "Name";
                            x: 2px;
                            y: -25px;
                            font-size: 20px;
                        }
                        TextInput {
                            x: 3px;
                            y: 0px;
                            width: 60px;
                            height: 25px;
                            z:90;
                            edited => {
                                CastNameOUT = self.text;
                            }
                        }

                    }
                    Rectangle {
                        border-radius: 5px;
                        background: #c8cdd6;
                        x: 80px;
                        y: 50px;
                        width: 60px;
                        height: 25px;
                        z:90;
                        Text {
                            text: "Age";
                            x: 10px;
                            y: -25px;
                            font-size: 20px;
                        }
                        TextInput {
                            edited => {
                                CastAgeOUT = self.text;
                            }

                        }

                    }
                    Rectangle {
                        y: 50px;
                        x: 155px;
                        width: 60px;
                        height: 25px;
                        z:90;
                        background: #c8cdd6;
                        border-radius: 5px;
                        Text {
                            text: "Role";
                            x: 3px;
                            y: -25px;
                            font-size: 20px;
                        }
                        TextInput {
                            edited => {
                                CastRoleOUT = self.text;
                            }

                        }

                    }
                    ListView {
                        x: 5px;
                        y: 105px;
                        width: 210px;
                        height: 100px;
                        viewport-width: 1000px;
                        for data in CastListIN: Text {
                            text: data;
                            font-size: 14px;
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
    if pool.is_none() {
        //Exit on no pool found
        print!("Error: No pool found\n Exiting...");
        return;
    } else {
        gui_loop(app, pool.unwrap()).await;
    }
}
async fn gui_loop(app: MainGui, pool: MySqlPool) {
    let weak_app = app.as_weak();
    let weak_pool = pool.clone();
    app.on_eventOccured(move || {
        let app = weak_app.upgrade().unwrap();
        let pool = weak_pool.clone();
        let _ = spawn_local(async move {
            match app.get_Event().as_str() {
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
                    app.set_MovieList(model);
                }
                "SubmitButtonClicked" => {
                    //Get movie details from GUI
                    let movie_title = app.get_MovieTitleOUT();
                    let release_date = app.get_ReleaseDateOUT();
                    let format = app.get_FormatOUT();
                    let description = app.get_DescriptionOUT();
                    let review_score = app.get_ReviewOUT();
                    let cast_list = CAST_LIST.lock().unwrap();
                    //fill in the struct that is sent to database
                    let from_gui = fill_from_gui(
                        movie_title.to_string(),
                        release_date.to_string().parse().unwrap(),
                        format.to_string(),
                        description.to_string(),
                        cast_list.clone(),
                        review_score,
                        0,
                        "".to_string(),
                        "".to_string(),
                    );
                    //add movie to db
                    add_movie(from_gui, &pool).await;
                    populate_movie_list(app, pool).await;
                    cast_list.clear();
                }
                "AddMemberClicked" => {
                    let mut cast_list = CAST_LIST.lock().unwrap();
                    let cast_name = app.get_CastNameOUT();
                    let cast_age = app.get_CastAgeOUT();
                    let cast_role = app.get_CastRoleOUT();
                    // Push to the cast_list
                    cast_list.push(format!(
                        "{};{};{}",
                        cast_name.to_string(),
                        cast_age.to_string(),
                        cast_role.to_string()
                    ));
                    app.set_CastListIN(vec_str_to_model(cast_list.clone()));
                }
                "RemoveMemberClicked" => {
                    let mut cast_list = CAST_LIST.lock().unwrap();
                    cast_list.pop();
                    app.set_CastListIN(vec_str_to_model(cast_list.clone()));
                }
                "DeleteMovieClicked" => {
                    let movie_title = app.get_MovieTitleOUT().to_string();
                    delete_data(movie_title, &pool).await;
                    populate_movie_list(app, pool).await;
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
            app.set_MovieList(model);
        }
        "Release-Date" => {
            let search_term_int = search_term.parse::<i32>().unwrap(); // Convert search_term to i32
            let movie_list = filter_by_release(&pool, search_term_int).await;
            let model = parse_movie_list(movie_list);
            app.set_MovieList(model);
        }
        "Format" => {
            let movie_list = filter_by_format(&pool, search_term).await;
            let model = parse_movie_list(movie_list);
            app.set_MovieList(model);
        }
        "Rating" => {
            let search_term_int = search_term.parse::<i32>().unwrap(); // Convert search_term to i32
            let movie_list = filter_by_rating(&pool, search_term_int).await;
            let model = parse_movie_list(movie_list);
            app.set_MovieList(model);
        }
        "Actor-Name" => {
            let search_term = search_term.to_string();
            println!("Actor-Name: {}", search_term);
            let movie_list = filter_by_actor(&pool, search_term).await;
            let model = parse_movie_list(movie_list);
            app.set_MovieList(model);
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
    if !result.MovieData.is_empty() {
        app.set_MovieTitleIN(string_to_shared_string(
            result.MovieData[0].title.clone().unwrap(),
        ));
        app.set_Format(string_to_shared_string(
            result.MovieData[0].format.clone().unwrap(),
        ));
        app.set_Description(string_to_shared_string(
            result.MovieData[0].description.clone().unwrap(),
        ));
        if !result.ActorData.is_empty() {
            app.set_Cast(string_to_shared_string(actorlist_to_string(
                result.ActorData.clone(),
            )));
        } else {
            println!("Warning: ActorData is empty");
        }
        app.set_ReleaseDate(result.MovieData[0].releaseDate.unwrap());
        println!("{}", actorlist_to_string(result.ActorData.clone()));
        if !result.ReviewData.is_empty() {
            if let Some(aggregate) = &result.ReviewData[0].aggregate {
                app.set_Review(string_to_shared_string(aggregate.to_string()));
                populate_sub_review_list(app, pool, result.ReviewData[0].reviewID.unwrap()).await;
            } else {
                println!("Warning: aggregate field is missing in ReviewData[0]");
                app.set_Review(string_to_shared_string("0".to_string()));
            }
        } else {
            println!("Warning: ReviewData is empty");
        }
    } else {
        println!("Warning: MovieData is empty");
    }
}

async fn populate_movie_list(app: MainGui, pool: MySqlPool) {
    let result = get_all(&pool).await;
    let model = parse_result(result);
    app.set_MovieList(model);
}

async fn populate_sub_review_list(app: MainGui, pool: MySqlPool, review_id: i32) {
    let result = get_sub_review_list(&pool, review_id).await;
    let model = parse_result(result);
    print!("SubReviewList: {:?}", model);
    app.set_SubReviewList(model);
}

fn fill_from_gui(
    movie_title: String,
    release_date: i32,
    format: String,
    description: String,
    cast: Vec<String>,
    aggregate: i32,
    sub_review_num: i32,
    sub_review_title: String,
    sub_review_desc: String,
) -> FromGui {
    //fill in the struct

    //brake cast array into elements
    let mut actor_name: Vec<String> = Vec::new();
    let mut actor_age: Vec<i32> = Vec::new();
    let mut actor_role: Vec<String> = Vec::new();

    for i in 0..cast.len() {
        let cast_member = cast.get(i).unwrap();
        let cast_member_split: Vec<&str> = cast_member.split(";").collect();

        actor_name.push(cast_member_split[0].to_string());

        let actorage: Result<i32, _> = cast_member_split[1].trim().parse();

        if let Ok(age) = actorage {
            print!("Actor Age: {}", age);
            actor_age.push(age);
        } else {
            println!("Invalid age for actor: {}", cast_member_split[1]);
            actor_age.push(0);
        }

        actor_role.push(cast_member_split[2].to_string());
    }

    FromGui {
        title: movie_title,
        actor_name: actor_name,
        actor_age: actor_age,
        actor_role: actor_role,
        aggregate: aggregate,
        description: description,
        format: format,
        releaseDate: release_date,
        sub_review_num: sub_review_num,
        sub_review_score: aggregate,
        sub_review_title: sub_review_title,
        sub_review_desc: sub_review_desc,
    }
}
