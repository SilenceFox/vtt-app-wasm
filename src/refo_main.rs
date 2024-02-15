// This is a copy of my testing code.
// You can use this as reference when testing it.
// Kissus. 

#[component]
fn Home(cx: Scope) -> Element {
    render! {
        // Put all of this inside a flex container
        div { class: "container",
            div { width: "50%", class: "theme",
                button {class: "box
                                top
                                br bl
                                centerh","Top" }
                button {class: "box
                                left
                                tr br","Left" }
                button {class: "box
                                right
                                tl bl","Right" }
                button {class: "box
                                bottom
                                tr tl
                                centerh","Bottom" }
            }
            div { width: "50%", class: "theme",
                button {class: "box2
                                top
                                br bl
                                centerh
                                overlay","Top" }
                button {class: "box2
                                left
                                tr br
                                overlay","Left" }
                button {class: "box2
                                right
                                tl bl
                                overlay","Right" }
                button {class: "box2
                                bottom
                                tr tl
                                centerh
                                overlay","Bottom" }
            }
        }

    }
}
