//! Minimal example: Custom Panel with DockArea
//!
//! Demonstrates:
//!   1. Define a custom panel by implementing the Panel trait
//!   2. Register it with register_panel()
//!   3. Add it to a DockArea

use gpui::*;
use gpui_component::{
    Root,
    dock::{
        ClosePanel, DockArea, DockItem, Panel, PanelControl, PanelEvent, PanelState,
        register_panel, ToggleZoom,
    },
};
use std::sync::Arc;

/// The custom panel entity
pub struct MyPanel {
    focus_handle: FocusHandle,
}

impl MyPanel {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl EventEmitter<PanelEvent> for MyPanel {}

impl Focusable for MyPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MyPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_4().child("Hello from MyPanel")
    }
}

impl Panel for MyPanel {
    fn panel_name(&self) -> &'static str {
        "MyPanel"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        "MyPanel"
    }

    fn closable(&self, _cx: &App) -> bool {
        true
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        Some(PanelControl::Both)
    }

    fn dump(&self, _cx: &App) -> PanelState {
        PanelState::new(self)
    }
}

/// Register MyPanel so it can be deserialized from saved layouts.
fn register_panels(cx: &mut App) {
    register_panel(cx, "MyPanel", |_dock_area, _state, _info, window, cx| {
        Box::new(cx.new(|cx| MyPanel::new(window, cx)))
    });
}

// ---------------------------------------------------------------------------
// Workspace that holds a DockArea and wires up panels
// ---------------------------------------------------------------------------

const DOCK_AREA_ID: &str = "demo-dock";
const DOCK_AREA_VERSION: usize = 1;

pub struct Workspace {
    dock_area: Entity<DockArea>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create the DockArea
        let dock_area = cx.new(|cx| {
            DockArea::new(DOCK_AREA_ID, Some(DOCK_AREA_VERSION), window, cx)
        });
        let weak_dock = dock_area.downgrade();

        // Build a center panel from our custom panel
        let center = DockItem::tabs(
            vec![Arc::new(cx.new(|cx| MyPanel::new(window, cx)))],
            &weak_dock,
            window,
            cx,
        );

        // Set the center content
        dock_area.update(cx, |area, cx| {
            area.set_center(center, window, cx);
        });

        Self { dock_area }
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(self.dock_area.clone())
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);
        register_panels(cx);

        cx.bind_keys(vec![
            KeyBinding::new("shift-escape", ToggleZoom, None),
            KeyBinding::new("ctrl-w", ClosePanel, None),
        ]);

        cx.activate(true);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| Workspace::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
