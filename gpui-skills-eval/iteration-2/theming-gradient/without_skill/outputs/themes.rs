use std::rc::Rc;

use gpui::App;
use gpui_component::{Theme, ThemeConfig};

/// Apply the Ocean Sunset dark theme with gradient primary backgrounds.
///
/// Uses `Theme::global_mut(cx).apply_config()` to set:
/// - Primary: `linear-gradient(135deg, #0EA5E9, #8B5CF6)`
/// - Background: `#0F172A`
/// - Foreground: `#F8FAFC`
/// - Hover/active states use slightly different gradient angles.
pub fn apply_ocean_sunset_theme(cx: &mut App) {
    let config = serde_json::from_value::<ThemeConfig>(serde_json::json!({
        "name": "Ocean Sunset",
        "mode": "dark",
        "colors": {
            "background": "#0F172A",
            "foreground": "#F8FAFC",
            "primary.background": "linear-gradient(135deg, #0EA5E9, #8B5CF6)",
            "primary.hover.background": "linear-gradient(150deg, #0EA5E9, #8B5CF6)",
            "primary.active.background": "linear-gradient(120deg, #0EA5E9, #8B5CF6)",
            "primary.foreground": "#FFFFFF",
            "secondary.background": "#1E293B",
            "secondary.foreground": "#CBD5E1",
            "muted.background": "#1E293B",
            "muted.foreground": "#94A3B8",
            "border": "#334155",
            "input.border": "#475569",
            "ring": "#0EA5E9",
        }
    }))
    .expect("Failed to parse Ocean Sunset theme config");

    Theme::global_mut(cx).apply_config(&Rc::new(config));
    cx.refresh_windows();
}
