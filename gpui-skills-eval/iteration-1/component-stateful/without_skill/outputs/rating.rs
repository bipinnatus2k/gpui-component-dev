use crate::theme::ActiveTheme;
use crate::{Disableable, Icon, IconName, Sizable, Size, StyledExt, h_flex};
use std::rc::Rc;

use gpui::{
    App, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
};
use gpui::{ClickEvent, Hsla, MouseMoveEvent, StatefulInteractiveElement};

pub fn init(_: &mut App) {}

/// Events emitted by [`RatingState`].
#[derive(Clone, Debug, PartialEq)]
pub enum RatingEvent {
    /// The rating value changed.
    Change(usize),
}

/// State for the Rating component.
pub struct RatingState {
    value: usize,
    hovered_value: usize,
    max: usize,
}

impl EventEmitter<RatingEvent> for RatingState {}

impl RatingState {
    /// Create a new [`RatingState`] with the given max and initial value.
    pub fn new(max: usize, value: usize) -> Self {
        Self {
            value: value.min(max),
            hovered_value: 0,
            max,
        }
    }

    /// Return the current rating value.
    pub fn value(&self) -> usize {
        self.value
    }

    /// Return the hovered star index (0 if not hovering).
    pub fn hovered_value(&self) -> usize {
        self.hovered_value
    }

    /// Return the maximum number of stars.
    pub fn max_value(&self) -> usize {
        self.max
    }

    /// Set the rating value and emit a [`RatingEvent::Change`].
    pub fn set_value(&mut self, value: usize, cx: &mut Context<Self>) {
        let v = value.min(self.max);
        if self.value != v {
            self.value = v;
            cx.emit(RatingEvent::Change(v));
            cx.notify();
        }
    }

    /// Set the hovered star index.
    pub fn set_hovered_value(&mut self, value: usize, cx: &mut Context<Self>) {
        self.hovered_value = value;
        cx.notify();
    }

    /// Reset the hovered value (typically on mouse leave).
    pub fn reset_hover(&mut self, cx: &mut Context<Self>) {
        self.hovered_value = 0;
        cx.notify();
    }
}

/// A star Rating element with managed state.
#[derive(IntoElement)]
pub struct Rating {
    state: Entity<RatingState>,
    style: StyleRefinement,
    size: Size,
    disabled: bool,
    color: Option<Hsla>,
    show_value: bool,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl Rating {
    /// Create a new [`Rating`] bound to a [`RatingState`].
    pub fn new(state: &Entity<RatingState>) -> Self {
        Self {
            state: state.clone(),
            style: StyleRefinement::default(),
            size: Size::Medium,
            disabled: false,
            color: None,
            show_value: false,
            on_click: None,
        }
    }

    /// Set the star icon size.
    pub fn star_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the star icon size.
    pub fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Display the numeric value alongside the stars.
    pub fn show_value(mut self) -> Self {
        self.show_value = true;
        self
    }

    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set active star color, defaults to theme yellow.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Add on_click handler when the rating changes.
    pub fn on_click(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for Rating {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Sizable for Rating {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Disableable for Rating {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Rating {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let entity_id = self.state.entity_id();
        let state = self.state;
        let size = self.size;
        let show_value = self.show_value;
        let on_click = self.on_click;
        let active_color = self.color.unwrap_or(cx.theme().yellow);

        let read = state.read(cx);
        let value = read.value;
        let max = read.max;
        let hovered_value = read.hovered_value;

        h_flex()
            .id(("rating", entity_id))
            .flex_nowrap()
            .items_center()
            .gap_1()
            .refine_style(&self.style)
            .on_hover(window.listener_for(&state, move |state, hovered: &bool, _, cx| {
                if !*hovered {
                    state.reset_hover(cx);
                }
            }))
            .children((1..=max).map(move |ix| {
                let state = state.clone();
                let on_click = on_click.clone();
                let filled = ix <= value;
                let hovered = hovered_value >= ix;

                div()
                    .id(ix)
                    .p_0p5()
                    .flex_none()
                    .flex_shrink_0()
                    .when(filled || hovered, |this| this.text_color(active_color))
                    .child(
                        Icon::new(if filled {
                            IconName::StarFill
                        } else {
                            IconName::Star
                        })
                        .with_size(size),
                    )
                    .when(!self.disabled, |this| {
                        this.on_mouse_move(window.listener_for(
                            &state,
                            move |state, _: &MouseMoveEvent, _, cx| {
                                state.set_hovered_value(ix, cx);
                            },
                        ))
                        .on_click(window.listener_for(
                            &state,
                            move |state, _: &ClickEvent, window, cx| {
                                let new = if state.value >= ix {
                                    ix.saturating_sub(1)
                                } else {
                                    ix
                                };
                                state.set_value(new, cx);
                                if let Some(ref on_click) = on_click {
                                    on_click(&new, window, cx);
                                }
                            },
                        ))
                    })
            }))
            .when(show_value, |this| {
                this.child(
                    div()
                        .ml_1()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("{}/{}", value, max)),
                )
            })
    }
}
