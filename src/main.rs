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
            div { "jaguara" },
            div { "jaguara" },
            div { "jaguara" },
            div { "jaguara" },
            div { "jaguara" },
            div { "jaguara" },
            div { "jaguara" },
            div { "jaguara" },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct NotificationData {
    id: u32,
    title: String,
    message: String,
}

#[component]
fn NotificationBar(cx: Scope) -> Element {
    let title = use_state(cx, || "Burro".to_string());
    let message = use_state(cx, || "Nesse exato momento esse texto deve dar overflow.".to_string());
    let counter = use_state(cx, || 0);
    
    render!(
        div { class: "notifications",
            button { onclick: move |_| counter.set(counter.saturating_add(1)), "Add" }
            button { onclick: move |_| counter.set(counter.saturating_sub(1)), "Remove" }
            // Notification(cx, title.as_str(), message.as_str()) {}
                for id in 0.. **counter {
                    Notification { id: id, title: "Message".into(), message: "Nesse exato momento esse texto deve dar overflow.".into() }
                }

            form { input { oninput: move |event| title.set(event.value.clone()), placeholder: "Title" } },
            form { input { oninput: move |msg| message.set(msg.value.clone()), placeholder: "Message" } },
        }
    )
}

#[component]
fn Notification(cx: Scope<NotificationData>) -> Element {
    render!(
        div {class: "child", id: "{cx.props.id}",
            div{ class: "title", {cx.props.title.as_str()}}
            div{ class: "message", {cx.props.message.as_str()}}
        }
    )
}

#[component]
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
