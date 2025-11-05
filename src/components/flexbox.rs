use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FlexDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, PartialEq)]
pub enum FlexAlignment {
    Start,
    End,
    Center,
    Stretch,
}

#[derive(Props, Clone, PartialEq)]
pub struct FlexboxProps {
    direction: FlexDirection,
    alignment: FlexAlignment,

    children: Element,
}

#[component]
pub fn Flexbox(props: FlexboxProps) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: match props.direction {
                FlexDirection::Vertical => "column",
                FlexDirection::Horizontal => "row",
            },
            align_items: match props.alignment {
                FlexAlignment::Start => "flex-start",
                FlexAlignment::End => "flex-end",
                FlexAlignment::Center => "center",
                FlexAlignment::Stretch => "stretch",
            },

            {props.children}
        }
    }
}
