use gpui::{div, prelude::*, rgb, App, IntoElement, SharedString, Window};

#[derive(IntoElement)]
pub struct Container {
    pub text: SharedString,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            text: "Button".into(),
        }
    }
}

impl RenderOnce for Container {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .border_4()
            .border_color(rgb(0x1E1E1F))
            .child(
                div()
                    .border_4()
                    .border_color(rgb(0x6A6C70))
                    .bg(rgb(0x48494A))
                    .p_2()
                    .child(format!("Hello, {}!", self.text)),
            )
            .child(div().bg(rgb(0x313233)).p_1())
    }
}
