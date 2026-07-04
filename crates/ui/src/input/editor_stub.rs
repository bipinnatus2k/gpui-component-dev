//! No-op stub types for code editor functionality when `code-editor` is disabled.

use gpui::{App, Bounds, Context, Window, HighlightStyle, IntoElement, AnyElement, Point, Pixels, SharedString};
use ropey::Rope;
use std::ops::Range;

// ---- LSP stub ----

#[derive(Clone)]
pub struct Lsp {
    pub code_action_providers: Vec<()>,
    pub definition_provider: Option<()>,
    pub completion_provider: Option<()>,
}

impl Lsp {
    pub fn default() -> Self {
        Self {
            code_action_providers: vec![],
            definition_provider: None,
            completion_provider: None,
        }
    }
    pub fn reset(&mut self) {}
    pub fn update(&mut self, _text: &Rope, _window: &mut Window, _cx: &mut Context<crate::input::InputState>) {}
    #[allow(unused)]
    pub fn semantic_tokens_for_range(&self, _text: &Rope, _visible_range: &Range<usize>, _theme: &crate::highlighter::HighlightTheme) -> Vec<(Range<usize>, HighlightStyle)> {
        vec![]
    }
    #[allow(unused)]
    pub fn document_colors_for_range(&self, _text: &Rope, _visible_range: &Range<usize>) -> Vec<(Range<usize>, gpui::Hsla)> {
        vec![]
    }
}

impl Default for Lsp {
    fn default() -> Self { Self::default() }
}

// ---- Stub entity types ----

/// Stub for DiagnosticPopover. Never actually created without code-editor.
pub struct DiagnosticPopover;
impl gpui::Render for DiagnosticPopover {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
    }
}

/// Stub for HoverPopover. Never actually created without code-editor.
#[derive(Clone)]
pub struct HoverPopover {
    pub symbol_range: Range<usize>,
}
impl Default for HoverPopover {
    fn default() -> Self { Self { symbol_range: 0..0 } }
}
impl gpui::Render for HoverPopover {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
    }
}

/// Stub for hover-based "Go to Definition" feature.
#[derive(Clone, Default)]
pub struct HoverDefinition;

/// Stub for inline completion item.
#[derive(Clone)]
pub struct InlineCompletionItem {
    pub insert_text: SharedString,
}

/// Stub for inline completions (ghost text).
#[derive(Clone, Default)]
pub struct InlineCompletion {
    pub item: Option<InlineCompletionItem>,
    pub range: Option<Range<usize>>,
}

/// Stub for completion/code-action context menu.
#[derive(Clone)]
pub enum ContextMenu {
    None,
}
impl ContextMenu {
    pub fn render(&self) -> gpui::AnyElement {
        gpui::div().into_any_element()
    }
}
