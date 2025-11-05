use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    text: String,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            style: "border: 1px solid black; width: auto; border-radius: 1em; padding: 0.5em 1em; margin: 0.5em;",

            {props.text}
        }
    }
}
