use anyhow::Result;
use gpui::*;
use gpui_component::{
    ActiveTheme, Root,
    dock::{
        DockArea, DockItem, Panel, PanelEvent, PanelInfo, PanelState, register_panel,
    },
};

actions!(my_app, [Quit]);

// ── 1. Define a custom panel by implementing the `Panel` trait ──────────

struct MyPanel {
    focus_handle: FocusHandle,
}

impl MyPanel {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Panel for MyPanel {
    fn panel_name(&self) -> &'static str {
        "MyPanel"
    }

    fn tab_name(&self, _cx: &App) -> Option<SharedString> {
        Some("MyPanel".into())
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        "MyPanel".to_string()
    }

    fn closable(&self, _cx: &App) -> bool {
        true
    }

    fn dump(&self, _cx: &App) -> PanelState {
        let mut state = PanelState::new(self);
        state.info = PanelInfo::panel(serde_json::Value::Null);
        state
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
        div()
            .size_full()
            .p_4()
            .child(div().child("Hello from MyPanel"))
    }
}

// ── 2. Register the panel with register_panel() ─────────────────────────

fn init_my_panel(cx: &mut App) {
    register_panel(cx, "MyPanel", |_, _, _info, window, cx| {
        let view = cx.new(|cx| MyPanel::new(window, cx));
        Box::new(view)
    });
}

// ── 3. Create a DockArea and add MyPanel ────────────────────────────────

struct AppView {
    dock_area: Entity<DockArea>,
}

impl AppView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let dock_area = cx.new(|cx| DockArea::new("my-dock-area", Some(1), window, cx));
        let weak = dock_area.downgrade();

        // Create MyPanel entity
        let my_panel = cx.new(|cx| MyPanel::new(window, cx));

        // Wrap it in a DockItem::tab (single-tab Tabs layout)
        let center = DockItem::tab(my_panel, &weak, window, cx);

        // Set as the center content of the DockArea
        dock_area.update(cx, |area, cx| {
            area.set_center(center, window, cx);
        });

        Self { dock_area }
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .font_family(cx.theme().font_family.clone())
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.dock_area.clone())
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}

// ── Application entry point ─────────────────────────────────────────────

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        init_my_panel(cx);

        cx.on_action(|_: &Quit, cx: &mut App| cx.quit());

        cx.on_action(|_: &my_app::Quit, cx: &mut App| cx.quit());
        cx.bind_keys([KeyBinding::new("escape", my_app::Quit, None)]);

        cx.activate(true);

        cx.spawn(async move |cx| {
            let window_bounds = Bounds::centered(
                None,
                size(px(800.), px(600.)),
                cx,
            );

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let app_view = cx.new(|cx| AppView::new(window, cx));
                    cx.new(|cx| Root::new(app_view, window, cx))
                },
            )
            .expect("failed to open window");
        })
        .detach();
    });
}
