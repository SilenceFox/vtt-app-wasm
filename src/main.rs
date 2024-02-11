#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_html_macro::html;
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
            div { id: "main",
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
        div { class: "overview", div { "jaguara" } }
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
        div { class: "notifications",
            div { width: "100%",
                NotificationContainer {}
                form { input { oninput: move |event| title.set(event.value.clone()), placeholder: "Title" } }
                form { 
                    input {
                        oninput: move |messag| message.set(messag.value.clone()),
                        placeholder: "Message"
                    } 
                }
                div {
                    "{title}"
                    br {}
                    "{message}"
                }
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
    render!( NotificationChild(cx, message_data), {} )

}

fn NotificationChild(cx: Scope, message: NotificationData) -> Element {
    render!(
        div {
            div { class: "title", "{message.title}" }
            div { class: "message", "{message.message}" }
        }
    )
}

fn ChatContainer(cx: Scope) -> Element {
    cx.render(html!(
        <div class="chat-container">
            <div class="chat-container-inner">
                <div class="chat-message"></div>
            </div>
        </div>

    ))
}
