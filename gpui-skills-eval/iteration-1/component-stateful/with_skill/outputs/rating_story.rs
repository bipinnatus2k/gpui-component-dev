use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::{
    ActiveTheme, IconName, Selectable as _, Sizable as _, Size,
    button::{Button, ButtonGroup},
    h_flex,
    rating::{Rating, RatingEvent, RatingState},
    v_flex,
};

use crate::section;

pub struct RatingStory {
    focus_handle: gpui::FocusHandle,
    size: Size,
    state: Entity<RatingState>,
}

impl super::Story for RatingStory {
    fn title() -> &'static str {
        "Rating"
    }

    fn description() -> &'static str {
        "A simple interactive star rating component."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl RatingStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|_| RatingState::new(5, 3));
        cx.subscribe(&state, |_, _: Entity<RatingState>, _: &RatingEvent, cx| {
            cx.notify();
        });
        Self {
            focus_handle: cx.focus_handle(),
            size: Size::default(),
            state,
        }
    }
}

impl Focusable for RatingStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

pub fn init(_cx: &mut App) {
    // No global init required for RatingStory
}

impl Render for RatingStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let disabled_state = cx.new(|_| RatingState::new(5, 2));
        let value = self.state.read(cx).value();

        v_flex()
            .w_full()
            .gap_3()
            .child(
                h_flex().w_full().gap_3().child(
                    ButtonGroup::new("toggle-size")
                        .outline()
                        .compact()
                        .child(
                            Button::new("xsmall")
                                .label("XSmall")
                                .selected(self.size == Size::XSmall),
                        )
                        .child(
                            Button::new("small")
                                .label("Small")
                                .selected(self.size == Size::Small),
                        )
                        .child(
                            Button::new("medium")
                                .label("Medium")
                                .selected(self.size == Size::Medium),
                        )
                        .child(
                            Button::new("large")
                                .label("Large")
                                .selected(self.size == Size::Large),
                        )
                        .on_click(cx.listener(|this, selecteds: &Vec<usize>, _, cx| {
                            let size = match selecteds[0] {
                                0 => Size::XSmall,
                                1 => Size::Small,
                                2 => Size::Medium,
                                3 => Size::Large,
                                _ => unreachable!(),
                            };
                            this.size = size;
                            cx.notify();
                        })),
                ),
            )
            .child(
                section("Basic Rating").max_w_md().child(
                    v_flex()
                        .w_full()
                        .gap_3()
                        .justify_center()
                        .items_center()
                        .child(
                            Rating::new(&self.state)
                                .with_size(self.size),
                        )
                        .child(
                            h_flex()
                                .gap_x_2()
                                .child(
                                    Button::new("r-dec")
                                        .small()
                                        .outline()
                                        .icon(IconName::Minus)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            let mut v = this.state.read(cx).value();
                                            v = v.saturating_sub(1);
                                            this.state.update(cx, |state, cx| {
                                                state.set_value(v, cx);
                                            });
                                        })),
                                )
                                .child(
                                    Button::new("r-inc")
                                        .small()
                                        .outline()
                                        .icon(IconName::Plus)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            let mut v = this.state.read(cx).value();
                                            v = (v + 1).min(5);
                                            this.state.update(cx, |state, cx| {
                                                state.set_value(v, cx);
                                            });
                                        })),
                                ),
                        ),
                ),
            )
            .child(
                section("Disabled").max_w_md().child(
                    Rating::new(&disabled_state)
                        .with_size(self.size)
                        .color(cx.theme().green)
                        .disabled(true),
                ),
            )
            .child(
                section("Custom Color").max_w_md().child(
                    Rating::new(&self.state)
                        .large()
                        .color(cx.theme().green),
                ),
            )
    }
}
