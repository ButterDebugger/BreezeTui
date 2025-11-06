use gpui::{
    div, prelude::*, rgb, App, Div, ElementId, Interactivity, IntoElement, MouseButton,
    MouseDownEvent, MouseUpEvent, SharedString, Stateful, Window,
};

#[derive(IntoElement)]
pub struct Button {
    pub text: SharedString,
    pub focused: bool,
}

impl Button {
    pub fn new(text: SharedString) -> Self {
        Self {
            text,
            focused: false,
        }
    }
}

// impl Button {
//     fn on_mouse_down(
//         &mut self,
//         _event: &MouseDownEvent,
//         _window: &mut Window,
//         _: &mut Context<Self>,
//     ) {
//         self.focused = true;

//         println!("Clicked!");
//     }

//     fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
//         self.focused = false;

//         println!("Released!");
//     }
// }

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .border_4()
            .border_color(rgb(0x1E1E1F))
            .cursor_pointer()
            .child(
                div()
                    .border_4()
                    .border_color(rgb(0x6A6C70))
                    .bg(rgb(0x48494A))
                    .p_2()
                    .child(format!("Hello, {}!", self.text)),
            )
            .child(div().bg(rgb(0x313233)).when_else(
                self.focused,
                |this| this.p_0p5(),
                |this| this.p_1(),
            ))
        // .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
        // .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
        // .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
    }
}
