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
    render! { Home {} }
}

#[component]
fn Home(cx: Scope) -> Element {
    render! {
        div { width: "100%", id: "app-window",
            nav { class: "nav",
                button { class: "button_done", "Home" }
                button { class: "button_done", "Sound" }
                button { class: "wip", "wip" }
                button { class: "wip", "wip" }
            }
            div { width: "100%", max_height: "100%", height: "93vh", id: "main", display: "flex",
                div { z_index: "50", id: "sidepanel", Sidebar {} }
                div {
                    id: "main-window",
                    width: "100%",
                    align_self: "left",
                    margin: "10px 10px 10px -20px",
                    box_shadow: "0 0 10px rgba(0, 0, 0, 1)",
                    Overview {}
                }
                NotificationBar {}
            }
        }
    }
}
#[component]
fn Sidebar(cx: Scope) -> Element {
    render! {
        div { class: "sidebar", id: "bar", align_self: "left",
            // Call on Overview when on Overview button focus
            button { class: "side_button", "Overview" }
            button { class: "side_button", "Scene" }
            button { class: "side_button", "Sheet" }
            button { class: "side_button", "Skills" }
            button { class: "side_button", "Notes" }
            div { id: "chat" }
        }
    }
}

#[component]
fn Overview(cx: Scope) -> Element {
    render! {
        div {
            height: "100%",
            width: "100%",
            border_radius: "16px",
            background: "#202080",
            color: "white",
            align_self: "left",
            hr {}
        }
    }
} 

#[component]
fn NotificationBar(cx: Scope) -> Element {
    render!(
        div {
            background: "#202080",
            id: "notifications",
            color: "white",
            margin: "10px 10px 10px 0px",
            padding: "0 0 10px 0",
            border_radius: "0 0 16px 16px",
            width: "30%",
            hr{}
            ol { width: "100%",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                flex_direction: "column",
                li{
                    margin: "5px 0",
                    "jaguara"
                },
                li{
                    margin: "5px 0",
                    "jaguara"
                },
                li{
                    margin: "5px 0",
                    "jaguara"
                },
            }
            hr{}
        }
    )
}
