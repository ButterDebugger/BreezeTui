use crate::views::home::Home;
use crate::views::mods::Mods;
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        // document::Stylesheet { href: CSS }

        Router::<Route> {}
    }
}

#[derive(Routable, PartialEq, Clone)]
pub(crate) enum Route {
    #[route("/")]
    Home {},

    #[route("/mods")]
    Mods {},
    // #[route("/modpacks")]
    // Modpacks {},

    // #[route("/installations")]
    // Installations {},
}
