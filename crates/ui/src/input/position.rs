/// A position in text, consisting of a line number (0-based) and character column (0-based).
///
/// This mirrors `lsp_types::Position` but is always available regardless of
/// whether the `code-editor` feature is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Position {
    /// Zero-based line number.
    pub line: u32,
    /// Zero-based character offset in UTF-8 bytes.
    pub character: u32,
}

impl Position {
    /// Create a new `Position`.
    pub const fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

#[cfg(feature = "code-editor")]
impl From<lsp_types::Position> for Position {
    fn from(pos: lsp_types::Position) -> Self {
        Self {
            line: pos.line,
            character: pos.character,
        }
    }
}

#[cfg(feature = "code-editor")]
impl From<Position> for lsp_types::Position {
    fn from(pos: Position) -> Self {
        Self {
            line: pos.line,
            character: pos.character,
        }
    }
}
