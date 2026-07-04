/// The character used to mask password input fields.
pub(super) const MASK_CHAR: char = '•';

mod blink_cursor;
mod change;
mod clear_button;
mod cursor;
mod display_map;
mod editor_stub;
mod element;
mod indent;
mod input;
#[cfg(feature = "code-editor")]
mod lsp;
mod mask_pattern;
mod mode;
mod movement;
mod number_input;
mod otp_input;
mod position;
#[cfg(feature = "code-editor")]
pub(crate) mod popovers;
mod rope_ext;
mod search;
mod selection;
mod state;

pub(crate) use clear_button::*;
pub use cursor::*;
#[cfg(target_family = "wasm")]
pub use display_map::folding::Tree;
pub use display_map::{BufferPoint, DisplayMap, DisplayPoint};
#[cfg(feature = "code-editor")]
pub use display_map::FoldRange;
pub use indent::TabSize;
pub use input::*;
#[cfg(feature = "code-editor")]
pub use lsp::*;
#[cfg(feature = "code-editor")]
pub(crate) use popovers::{ContextMenu, DiagnosticPopover, HoverPopover};
#[cfg(not(feature = "code-editor"))]
pub(crate) use editor_stub::{ContextMenu, DiagnosticPopover, HoverPopover, Lsp, HoverDefinition, InlineCompletion};
pub use mask_pattern::MaskPattern;
pub use number_input::{NumberInput, NumberInputEvent, NumberStep, StepAction};
pub use otp_input::*;
#[cfg(feature = "code-editor")]
pub use lsp_types::Position;
#[cfg(not(feature = "code-editor"))]
pub use position::Position;
pub use rope_ext::{InputEdit, Point, RopeExt, RopeLines};
pub use ropey::Rope;
pub use state::*;
