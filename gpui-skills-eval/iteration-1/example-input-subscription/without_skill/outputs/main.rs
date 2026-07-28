use gpui::*;
use gpui_component::{
    input::{Input, InputEvent, InputState},
    *,
};
use gpui_component_assets::Assets;

pub struct Example {
    input_state: Entity<InputState>,
    display_text: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl Example {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Type something..."));

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value = input_state.read(cx).value();
                    this.display_text = if value.is_empty() {
                        SharedString::default()
                    } else {
                        format!("You typed: {}", value).into()
                    };
                    cx.notify()
                }
                _ => {}
            }
        })];

        Self {
            input_state,
            display_text: SharedString::default(),
            _subscriptions,
        }
    }
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_5()
            .gap_3()
            .size_full()
            .items_center()
            .justify_center()
            .child(Input::new(&self.input_state))
            .child(
                div()
                    .text_color(if self.display_text.is_empty() {
                        cx.theme().muted_foreground
                    } else {
                        cx.theme().foreground
                    })
                    .child(
                        if self.display_text.is_empty() {
                            "Waiting for input..."
                        } else {
                            self.display_text.as_str()
                        },
                    ),
            )
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Example::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
