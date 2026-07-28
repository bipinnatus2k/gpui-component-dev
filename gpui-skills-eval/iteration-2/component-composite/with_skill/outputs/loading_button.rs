use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
};

use crate::{
    Button, ButtonRounded, ButtonVariant, ButtonVariants, Disableable, Selectable, Sizable, Size,
    Spinner, StyledExt, h_flex,
};

/// A button that shows a loading spinner when in the loading state.
///
/// Wraps the existing `Button` component and adds loading state management.
/// When `loading` is `true`, the label is replaced with a `Spinner` and
/// optional spinner text.
///
/// # Example
///
/// ```ignore
/// use gpui_component::LoadingButton;
///
/// LoadingButton::new("save")
///     .label("Save")
///     .primary()
///     .loading(is_saving)
///     .spinner_text("Saving...")
///     .on_click(|_, _, _| {})
/// ```
#[derive(IntoElement)]
pub struct LoadingButton {
    id: ElementId,
    style: StyleRefinement,
    label: Option<SharedString>,
    loading: bool,
    spinner_text: Option<SharedString>,
    disabled: bool,
    selected: bool,
    variant: ButtonVariant,
    size: Size,
    outline: bool,
    compact: bool,
    rounded: ButtonRounded,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

impl LoadingButton {
    /// Create a new LoadingButton.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            label: None,
            loading: false,
            spinner_text: None,
            disabled: false,
            selected: false,
            variant: ButtonVariant::default(),
            size: Size::default(),
            outline: false,
            compact: false,
            rounded: ButtonRounded::default(),
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the button label.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set to `true` to show the loading spinner instead of the label.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set text shown alongside the spinner when loading.
    pub fn spinner_text(mut self, text: impl Into<SharedString>) -> Self {
        self.spinner_text = Some(text.into());
        self
    }

    /// Add click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Set the outline style.
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// Set the button to compact mode.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Set the border radius.
    pub fn rounded(mut self, rounded: impl Into<ButtonRounded>) -> Self {
        self.rounded = rounded.into();
        self
    }
}

impl Styled for LoadingButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for LoadingButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Disableable for LoadingButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for LoadingButton {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Sizable for LoadingButton {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl ButtonVariants for LoadingButton {
    fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for LoadingButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let button = if self.loading {
            let spinner_text = self.spinner_text;

            Button::new(self.id)
                .disabled(true)
                .selected(self.selected)
                .with_variant(self.variant)
                .with_size(self.size)
                .rounded(self.rounded)
                .when(self.outline, |this| this.outline())
                .when(self.compact, |this| this.compact())
                .when_some(self.on_click, |this, handler| this.on_click(handler))
                .refine_style(&self.style)
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .justify_center()
                        .child(Spinner::new().with_size(Size::Small))
                        .when_some(spinner_text, |this, text| {
                            this.child(div().child(text))
                        }),
                )
        } else {
            Button::new(self.id)
                .when_some(self.label, |this, label| this.label(label))
                .disabled(self.disabled)
                .selected(self.selected)
                .with_variant(self.variant)
                .with_size(self.size)
                .rounded(self.rounded)
                .when(self.outline, |this| this.outline())
                .when(self.compact, |this| this.compact())
                .when_some(self.on_click, |this, handler| this.on_click(handler))
                .refine_style(&self.style)
                .children(self.children)
        };

        button.into_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_loading_button_builder(_cx: &mut gpui::TestAppContext) {
        let button = LoadingButton::new("test-loading")
            .label("Submit")
            .primary()
            .loading(true)
            .spinner_text("Submitting...")
            .outline()
            .compact()
            .selected(false)
            .disabled(false)
            .rounded(ButtonRounded::Medium)
            .on_click(|_, _, _| {});

        assert_eq!(button.label, Some("Submit".into()));
        assert_eq!(button.variant, ButtonVariant::Primary);
        assert!(button.loading);
        assert_eq!(button.spinner_text, Some("Submitting...".into()));
        assert!(button.outline);
        assert!(button.compact);
        assert!(!button.selected);
        assert!(!button.disabled);
        assert!(matches!(button.rounded, ButtonRounded::Medium));
        assert!(button.on_click.is_some());
    }

    #[gpui::test]
    fn test_loading_button_not_loading(_cx: &mut gpui::TestAppContext) {
        let button = LoadingButton::new("test-normal")
            .label("Save")
            .danger()
            .loading(false);

        assert_eq!(button.label, Some("Save".into()));
        assert_eq!(button.variant, ButtonVariant::Danger);
        assert!(!button.loading);
        assert!(button.spinner_text.is_none());
    }
}
