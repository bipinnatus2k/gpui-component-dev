use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
    div, px, relative,
};

use crate::{ActiveTheme, Sizable, Size, StyledExt};

/// A simple text label with configurable size and muted variant.
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    style: StyleRefinement,
    size: Size,
    muted: bool,
}

impl Label {
    /// Create a new label with the given text.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
            size: Size::default(),
            muted: false,
        }
    }

    /// Render the label with muted (secondary) color.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

impl Sizable for Label {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Label {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(if self.muted {
                cx.theme().muted_foreground
            } else {
                cx.theme().foreground
            })
            .text_size(match self.size {
                Size::XSmall => px(12.),
                Size::Small => px(14.),
                Size::Medium => px(14.),
                Size::Large => px(16.),
                Size::Size(px) => px * 0.875,
            })
            .line_height(relative(1.25))
            .refine_style(&self.style)
            .child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_label_builder(_cx: &mut gpui::TestAppContext) {
        let label = Label::new("Hello World").muted().small();

        assert!(label.muted);
        assert_eq!(label.size, Size::Small);
        assert_eq!(label.text, "Hello World");
    }

    #[gpui::test]
    fn test_label_default_size(_cx: &mut gpui::TestAppContext) {
        let label = Label::new("Default");
        assert_eq!(label.size, Size::Medium);
        assert!(!label.muted);
    }

    #[gpui::test]
    fn test_label_size_methods(_cx: &mut gpui::TestAppContext) {
        assert_eq!(Label::new("xs").xsmall().size, Size::XSmall);
        assert_eq!(Label::new("sm").small().size, Size::Small);
        assert_eq!(Label::new("lg").large().size, Size::Large);
    }
}
