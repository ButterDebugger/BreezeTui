use dioxus::prelude::*;

#[component]
pub fn ModEntry() -> Element {
    rsx! {
        div {
            background_color: "red",

            span {
                "mod 1"
            }
        }
    }
}