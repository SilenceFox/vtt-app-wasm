#![allow(non_snake_case)]

use dioxus_router::prelude::*;

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
            background: linear-gradient(45deg, #80AAFF, #F0F0F0);
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
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[component]
fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    let button = css!("
            border: none;
            padding: 4px 16px;
            border-radius: 12px 12px 0px 0px;
            width: 25%;
            font-size: 20px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 1);
            
            background: deepskyblue;
            transition: background 0.2s ease-out;

            &:hover {
                background: cyan;
                cursor: pointer;
            }
    ");

    let container2 = css!("
        background: linear-gradient(45deg, #0000FF, #F0F0F0);
        display: flex;
        width: 100vw;
        gap: 10px;
        padding: 0 10px;
    ");

     let container = css!(
            "
            display: flex;
            flex-direction: deck;
            align-items: center;
            gap: 10px;
            "
    );

    cx.render(rsx! {
        div {
            class: "{container2}",
            button { class: button, "Lucas 1" }
            button { class: button, "Lucas 2" }
            button { class: button, "Lucas 3" }
            button { class: button, "Lucas 4" }
        }	

    })
}
