use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::{
    Disableable, Selectable, Sizable, Size, Spinner,
    StyledExt as _,
    button::{Button, ButtonRounded, ButtonVariant, ButtonVariants},
};

/// A button that shows a loading spinner when in the loading state.
///
/// Built on top of the [`Button`] component with additional loading support:
/// - When `loading` is true, the label is replaced with a spinner icon and optional spinner text
/// - The button becomes non-clickable during loading
/// - Supports all button variants (primary, danger, etc.)
///
/// # Example
///
/// ```ignore
/// use gpui_component::LoadingButton;
///
/// LoadingButton::new("save")
///     .label("Save")
///     .primary()
///     .loading(true)
///     .spinner_text("Saving...")
///     .on_click(|_, _, _| {})
/// ```
#[derive(IntoElement)]
pub struct LoadingButton {
    id: ElementId,
    style: StyleRefinement,
    label: Option<SharedString>,
    size: Size,
    variant: ButtonVariant,
    rounded: ButtonRounded,
    outline: bool,
    compact: bool,
    disabled: bool,
    selected: bool,
    loading: bool,
    spinner_text: Option<SharedString>,
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
            size: Size::Medium,
            variant: ButtonVariant::default(),
            rounded: ButtonRounded::Medium,
            outline: false,
            compact: false,
            disabled: false,
            selected: false,
            loading: false,
            spinner_text: None,
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the label text of the button.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the loading state. When true, the button shows a spinner instead of the label.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set optional text to show next to the spinner when loading.
    pub fn spinner_text(mut self, text: impl Into<SharedString>) -> Self {
        self.spinner_text = Some(text.into());
        self
    }

    /// Set the button to compact mode with reduced padding.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Set the button to outline style.
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// Set the border radius of the button.
    pub fn rounded(mut self, rounded: impl Into<ButtonRounded>) -> Self {
        self.rounded = rounded.into();
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

impl RenderOnce for LoadingButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if self.loading {
            let icon_size = match self.size {
                Size::Size(v) => Size::Size(v * 0.75),
                _ => self.size,
            };

            Button::new(self.id.clone())
                .with_variant(self.variant)
                .with_size(self.size)
                .rounded(self.rounded)
                .when(self.outline, |btn| btn.outline())
                .when(self.compact, |btn| btn.compact())
                .disabled(self.disabled)
                .selected(self.selected)
                .loading(true)
                .refine_style(&self.style)
                .when_some(self.on_click, |btn, handler| btn.on_click(handler))
                .icon(Spinner::new().with_size(icon_size))
                .when_some(self.spinner_text, |btn, text| {
                    btn.child(div().child(text))
                })
                .children(self.children)
                .into_element()
        } else {
            Button::new(self.id.clone())
                .when_some(self.label, |btn, label| btn.label(label))
                .with_variant(self.variant)
                .with_size(self.size)
                .rounded(self.rounded)
                .when(self.outline, |btn| btn.outline())
                .when(self.compact, |btn| btn.compact())
                .disabled(self.disabled)
                .selected(self.selected)
                .refine_style(&self.style)
                .when_some(self.on_click, |btn, handler| btn.on_click(handler))
                .children(self.children)
                .into_element()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_loading_button_builder(_cx: &mut gpui::TestAppContext) {
        let btn = LoadingButton::new("test-loading")
            .label("Save")
            .primary()
            .danger()
            .outline()
            .large()
            .compact()
            .loading(true)
            .spinner_text("Saving...")
            .disabled(false)
            .selected(false)
            .rounded(ButtonRounded::Medium)
            .on_click(|_, _, _| {});

        assert_eq!(btn.label, Some("Save".into()));
        assert_eq!(btn.variant, ButtonVariant::Danger);
        assert!(btn.outline);
        assert_eq!(btn.size, Size::Large);
        assert!(btn.compact);
        assert!(btn.loading);
        assert_eq!(btn.spinner_text, Some("Saving...".into()));
        assert!(!btn.disabled);
        assert!(!btn.selected);
        assert!(matches!(btn.rounded, ButtonRounded::Medium));
        assert!(btn.on_click.is_some());
    }

    #[gpui::test]
    fn test_loading_button_defaults(_cx: &mut gpui::TestAppContext) {
        let btn = LoadingButton::new("default");

        assert!(btn.label.is_none());
        assert!(!btn.loading);
        assert!(btn.spinner_text.is_none());
        assert_eq!(btn.variant, ButtonVariant::Default);
        assert_eq!(btn.size, Size::Medium);
        assert!(!btn.disabled);
        assert!(!btn.selected);
    }

    #[gpui::test]
    fn test_loading_button_loading_state_config(_cx: &mut gpui::TestAppContext) {
        let loading = LoadingButton::new("loading").loading(true);
        assert!(loading.loading);

        let not_loading = LoadingButton::new("not-loading").loading(false);
        assert!(!not_loading.loading);
    }

    #[gpui::test]
    fn test_loading_button_variant_methods(_cx: &mut gpui::TestAppContext) {
        let primary = LoadingButton::new("primary").primary();
        assert_eq!(primary.variant, ButtonVariant::Primary);

        let danger = LoadingButton::new("danger").danger();
        assert_eq!(danger.variant, ButtonVariant::Danger);
    }

    #[gpui::test]
    fn test_loading_button_spinner_text(_cx: &mut gpui::TestAppContext) {
        let btn = LoadingButton::new("spinner").spinner_text("Loading...");
        assert_eq!(btn.spinner_text, Some("Loading...".into()));

        let btn_empty = LoadingButton::new("no-spinner");
        assert!(btn_empty.spinner_text.is_none());
    }
}
