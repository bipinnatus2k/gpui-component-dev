use std::rc::Rc;

use gpui::App;
use gpui_component::{Theme, ThemeConfig};

/// Apply the Ocean Sunset dark theme programmatically via `ThemeConfig` JSON.
///
/// Uses `Theme::global_mut(cx).apply_config()` pattern to set:
/// - Primary background: `linear-gradient(135deg, #0EA5E9, #8B5CF6)`
/// - Background: `#0F172A`
/// - Foreground: `#F8FAFC`
/// - Hover/active variants with slightly different gradient angles
pub fn apply_ocean_sunset_theme(cx: &mut App) {
    let config = serde_json::from_value::<ThemeConfig>(serde_json::json!({
        "name": "Ocean Sunset",
        "mode": "dark",
        "radius": 8,
        "colors": {
            "background": "#0F172A",
            "foreground": "#F8FAFC",
            "primary.background": "linear-gradient(135deg, #0EA5E9, #8B5CF6)",
            "primary.foreground": "#FFFFFF",
            "primary.hover.background": "linear-gradient(120deg, #0EA5E9, #8B5CF6)",
            "primary.active.background": "linear-gradient(150deg, #0EA5E9, #8B5CF6)",
        }
    }))
    .expect("Failed to parse Ocean Sunset theme config");

    Theme::global_mut(cx).apply_config(&Rc::new(config));
    cx.refresh_windows();
}
