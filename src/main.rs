#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

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
                button { class: "button", "Home" }
                button { class: "button", "Sound" }
                button { class: "wip", "wip" }
                button { class: "wip", "wip" }
            }
            div { id: "main",
                div { z_index: "50", id: "sidepanel", Sidebar {} }
                div { id: "main-window", Overview {} }
                NotificationBar {}
            }
        }
    }
}
#[component]
fn Sidebar(cx: Scope) -> Element {
    render! {
        div { class: "sidebar", id: "bar",
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
            div { "jaguara" }
        }
    }
}

struct NotificationData {
    title: String,
    message: String,
}
#[component]
fn NotificationBar(cx: Scope) -> Element {
    let title = use_state(cx, || "".to_string());
    let message = use_state(cx, || "".to_string());

    render!(
        div { id: "notifications",
            ol { width: "100%", id: "notification-container",
                li { margin: "5px 0", id: "notification-child", "jaguara" },
                NotificationContainer {},

                form { 
                    input {
                        oninput: move |event| title.set(event.value.clone()),
                        placeholder: "Title"
                    }
                }

                form {
                    input{ oninput: move |messag| message.set(messag.value.clone()),
                        placeholder: "Message"}
                }
                li { "{title}", br{} "{message}" }
            }
        }
    )
}

fn NotificationContainer(cx: Scope) -> Element {
    //dinamycally increase the number of notificationsChild inside the container
    let message_data = NotificationData {
        title: "Test".to_string(),
        message: "Very much Tested".to_string(),
    };
    render!( NotificationChild(cx, message_data) {})

}

fn NotificationChild(cx: Scope, message: NotificationData) -> Element {
    render!(
        li { class: "notification-child", id: "id",
            div { class: "notification-child", id: "title", "{message.title}" }
            div { class: "notification-child", id: "content", "{message.message}" }
        }
    )
}
