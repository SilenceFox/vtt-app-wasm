#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use sir::{AppStyle, css, global_css};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    global_css!("
        body {
            background: linear-gradient(180deg, #000044, #000000);
            height: 100vh;
            width: 100vw;
            display: flex;
            // flex-direction: deck;
            align-items: start;
            justify-content: start;}
        * { margin: 0; padding: 0; box-sizing: border-box;}
    ");
    render! {
        AppStyle {}
        Home {}
    }
}


#[component]
fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        "Blog post {id}"
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    let button = css!("
            color: darkslategray;
            border: none;
            padding: 4px 16px;
            border-radius: 0px 0px 12px 12px;
            width: 25%;
            height: 2em;
            font-size: 20px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 1);
            
            background: deepskyblue;
            transition: background 0.2s ease-out;
            transition: color 0.2s ease-out;
            transition: width 0.5s ease;

            &:hover {
                width: 30%;
                color: black;
                background: cyan;
                cursor: pointer;
            }
    ");

    let container2 = css!("
        background: darkslateblue;
        display: flex;
        width: 100vw;
        gap: 10px;
        padding: 0 10px;
    ");

    let wip = css!("
        color: black;
        border: none;
        padding: 4px 16px;
        border-radius: 0px 0px 12px 12px;
        width: 25%;
        font-size: 20px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 1);
        background: orange;
        transition: background 0.2s ease-out;
        &:hover {
            background: red;
            cursor: not-allowed;
            color: white;
        }"
    );


    render! {
    
        nav {
            class: "{container2}",
            button { class: button, "Home" }
            button { class: button, "Sound" }
            button { class: wip, "wip" }
            button { class: wip, "wip" }
        }
        Overview {}

    }
}
fn Overview(cx: Scope) -> Element {
     let container = css!(
        "
        display: flex;
        flex-direction: column;
        align-items: start;
        gap: 10px;
        "
    );
    let button = css!("
        background: #202088;
        color: white;
        border: none;
        padding: 4px 16px;
        border-radius: 0px 12px 12px 0px; 
        width: 100%;
        height: 2.5em;
        font-size: 20px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 1);
        transition: background 0.5s ease;
        transition: height 0.5s ease-out;
        &:hover {
            background: #AA0055;
            height: 3em;
        }
        &:clicked {  //
            background: #0000FF;
            height: 2.8em;
        }
        " //TODO: Gotta learn this, mainly how Dioxus handles events like toggling a clicked state.
    );
        
    render! {
        div {
            display: "flex",
            align_items: "left",
            div {
                class: "{container}",
                id: "bar",
                align_self:"center",
                width: "25%",
                height: "100%",
                transition: "height 0.5s ease-out",
                button { class: button, "Overview", },
                button { class: button, "Scene", },
                button { class: button, "Sheet", },
                button { class: button, "Skills", },
                button { class: button, "Notes", },
                div { id:"chat",}
            }
        }
    }
}
