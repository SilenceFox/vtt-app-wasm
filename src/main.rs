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
                button { "Home" }
                button { "Sound" }
                button { class: "wip", "wip" }
                button { class: "wip", "wip" }
            }
            div { id: "high",
                Sidebar {}
                Overview {}
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
            button { "Overview" }
            button { "Scene" }
            button { "Sheet" }
            button { "Skills" }
            button { "Notes" }
            div { id: "chat" }
        }
    }
}

#[component]
fn Overview(cx: Scope) -> Element {
    render! {
        div { class: "overview",
            div { "jaguara" }
            div { "jaguara" }
            div { "jaguara" }
            div { "jaguara" }
            div { "jaguara" }
            div { "jaguara" }
            div { "jaguara" }
            div { "jaguara" }
        }
    }
}

struct NotificationData {
    title: String,
    message: String,
}
impl NotificationData {
    pub fn new(title: &str, message: &str) -> NotificationData {
        let (title, message) = (title.to_string(), message.to_string());
        NotificationData { title, message }
    }
}
#[component]
fn NotificationBar(cx: Scope) -> Element {
    let title = use_state(cx, || "".to_string());
    let message = use_state(cx, || "".to_string());
    
    render!(
        div { class: "notifications",
            Notification(cx, title.as_str(), message.as_str()) {}
            form { input { oninput: move |event| title.set(event.value.clone()), placeholder: "Title" } }
            form { 
                input {
                    oninput: move |messag| message.set(messag.value.clone()),
                    placeholder: "Message"
                } 
            }
        }
    )
}

fn Notification<'a>(cx: Scope<'a>, title: &'a str, message: &'a str) -> Element<'a> {
    let notification = NotificationData::new(title, message);
    render!(
        div {class: "child",
            div{ class: "title", {notification.title}}
            div{ class: "message", {notification.message}}
        }
    )
}

fn ChatContainer(cx: Scope) -> Element {
    render!(
        div{
            class:"chat-container",
            div{ 
                class:"chat-container-inner",
                div{class:"chat-message"}
            }
        }
    )
}
