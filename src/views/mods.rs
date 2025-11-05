use crate::{
    components::{
        flexbox::{FlexAlignment, FlexDirection, Flexbox},
        mod_entry::ModEntry,
    },
    utils::get_mod_names,
};
use dioxus::prelude::*;

pub fn Mods() -> Element {
    rsx! {
        Flexbox {
            direction: FlexDirection::Horizontal,
            alignment: FlexAlignment::Center,

            button {
                "Edit"
            }
            button {
                "Clear"
            }
            button {
                "Reveal in File Explorer"
            }
        }
        Flexbox {
            direction: FlexDirection::Vertical,
            alignment: FlexAlignment::Stretch,

            // for modd in get_mod_names() {
            //     ModEntry {}
            // }
            div {
                background_color: "red",

                span {
                    "mod 1"
                }
            }
            div {
                background_color: "red",

                span {
                    "mod 2"
                }
            }
        }
    }
}
