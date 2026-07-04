//! Code editor functionality.
//!
//! This module is only available when the `code-editor` feature is enabled.
//! It provides the public API for code editor state, LSP integration,
//! and editing features like syntax highlighting, code folding, and line numbers.

// Re-export LSP traits from the input::lsp module.
pub use crate::input::{
    CodeActionProvider, CompletionProvider, DefinitionProvider,
    DocumentColorProvider, DocumentRangeSemanticTokensProvider, HoverProvider,
};

// Re-export LSP state types.
pub use crate::input::Lsp;

// Re-export display map types for code folding.
pub use crate::input::FoldRange;

// Re-export code-editor-specific actions.
pub use crate::input::{GoToDefinition, ToggleCodeActions};
