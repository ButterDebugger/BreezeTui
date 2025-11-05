use crate::components::{
    button::Button,
    flexbox::{FlexAlignment, FlexDirection, Flexbox},
};
use crate::views::app::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Flexbox {
            direction: FlexDirection::Vertical,
            alignment: FlexAlignment::Center,

            h1 {
                "Home"
            }
        }
        h1 {
            style: "color: red;",

            "Hello World"
        }
        Link {
            to: Route::Mods {},

            button {
                "Mods"
            }
        }
        Button {
            text: "wow".to_string()
        }
    }
}
