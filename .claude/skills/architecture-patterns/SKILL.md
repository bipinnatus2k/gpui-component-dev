---
name: architecture-patterns
description: Deep analysis of the gpui-component project architecture, including the Root view system, application initialization flow, crate organization, module structure, component categories (stateless vs stateful vs composite), Dock/panel layout system, keyboard navigation, and event dispatch. Use this when working with the overall architecture, understanding how components fit together, creating new crates, modifying the init flow, or making architectural decisions.
---

# Architecture & Patterns

## Project Architecture Overview

The gpui-component library is a Rust workspace project that provides 60+ cross-platform desktop UI components on top of [GPUI](https://gpui.rs) (Zed's GUI framework). It is inspired by macOS/Windows controls and shadcn/ui design patterns.

### Crate Organization

| Crate | Purpose | Published |
|-------|---------|-----------|
| `crates/ui` | Core UI component library | `gpui-component` on crates.io |
| `crates/macros` | Procedural macros (`IntoPlot` derive) | `gpui-component-macros` |
| `crates/story` | Desktop gallery app for showcasing components | Not published |
| `crates/story-web` | Web/WASM version of story gallery | Not published |
| `crates/assets` | Static assets (icons, SVGs) | `gpui-component-assets` |
| `crates/webview` | WebView component support | Not published |

### Workspace Dependencies (Cargo.toml)

All crates share:
- `gpui` — from Zed git repository (the core framework)
- `gpui_platform` — platform-specific GPUI features
- Keys: `serde`, `anyhow`, `schemars`, `rust-i18n`

**Important:** The project uses `edition = "2024"` (Rust edition 2024, not 2021).

### Clippy Lints

Notable lint allowances from workspace `Cargo.toml`:
- `dbg_macro = "deny"` — never commit `dbg!()` calls
- `todo = "deny"` — never commit `todo!()` macros
- `style = { level = "allow" }` — stylistic lints are fully disabled
- `type_complexity = "allow"` — complex types in signatures are OK

---

## Application Initialization Flow

Every application using gpui-component follows this exact sequence:

```
fn main()
  └─ gpui_platform::application()
       └─ .with_assets(Assets)  // only if using icons/images
            └─ .run(move |cx| {
                 gpui_component::init(cx);        // [1] REQUIRED first
                 cx.spawn(async move |cx| {       // [2] async context
                   cx.open_window(options, |w,cx| {
                     let view = cx.new(|_| Example);      // [3] create main view
                     cx.new(|cx| Root::new(view, w, cx))   // [4] wrap in Root
                   })
                 }).detach();                     // [5] detach the future
               })
```

**Key rules:**
- `gpui_component::init(cx)` **must** be the first call inside `app.run()` closure
- The root-level view of every window **must** be a `Root`
- Use `cx.spawn(async move {}).detach()` to open windows asynchronously
- Window options use `WindowOptions::default()` or customized `WindowBounds`

### What `gpui_component::init()` Initializes

Defined in `crates/ui/src/lib.rs:111`, it calls these in order:
1. `theme::init(cx)` — Theme singleton, light/dark modes
2. `global_state::init(cx)` — Global state management
3. `root::init(cx)` — Root system (dialogs, sheets, notifications, keyboard nav)
4. `focus_trap::init(cx)` — Focus trap system
5. `color_picker::init(cx)` — Color picker component
6. `date_picker::init(cx)` — (feature-gated) Date picker
7. `dock::init(cx)` — Dock/panel layout system
8. `sheet::init(cx)` — Sheet overlay system
9. `combobox::init(cx)`, `select::init(cx)`, `input::init(cx)` — Input components
10. `list::init(cx)`, `dialog::init(cx)`, `popover::init(cx)`, `menu::init(cx)`
11. `table::init(cx)`, `text::init(cx)`, `tree::init(cx)`, `tooltip::init(cx)`

Each `init()` typically registers key bindings, actions, and global state. For example, `dialog::init()` registers `CancelDialog` (Escape) and `ConfirmDialog` (Enter) keybindings.

---

## The Root View System

`Root` is defined in `crates/ui/src/root.rs` and is the **mandatory top-level view** for every window. It manages:

### Root Responsibilities

1. **Sheet layer** — side panels that slide in from edges
   - Access via `Root::render_sheet_layer(window, cx)`
   - Open sheets with `window.open_sheet()`
2. **Dialog layer** — modal dialogs
   - Access via `Root::render_dialog_layer(window, cx)`
   - Open dialogs with `window.open_dialog()`
3. **Notification layer** — toast notifications
   - Access via `Root::render_notification_layer(window, cx)`
   - Show notifications with `window.push_notification()`
4. **Keyboard navigation** — Tab/Shift-Tab cycling between focusable elements
5. **Context menu layer**
6. **Tooltip management**

### Rendering Layers in Custom Roots

If a component or example renders its own Root-like structure, it MUST include these layers:

```rust
div()
    .child(my_content)
    .children(Root::render_dialog_layer(window, cx))
    .children(Root::render_sheet_layer(window, cx))
    .children(Root::render_notification_layer(window, cx))
```

### WindowExt Extension

The `WindowExt` trait (in `crates/ui/src/window_ext.rs`) adds dialog/sheet/notification methods to `gpui::Window`:

| Method | Purpose |
|--------|---------|
| `window.open_dialog(cx, builder_fn)` | Opens a modal dialog |
| `window.close_dialog(cx)` | Closes the current dialog |
| `window.open_sheet(cx, builder_fn)` | Opens a side sheet |
| `window.close_sheet(cx)` | Closes the current sheet |
| `window.push_notification(cx, notification)` | Shows a notification toast |
| `window.open_alert_dialog(cx, alert, builder_fn)` | Opens an alert dialog |

---

## Component Categories

### 1. Stateless Components (RenderOnce)

These are pure presentation components. They use `#[derive(IntoElement)]`, implement `RenderOnce`, have no `Entity` state, and are rendered once per frame.

**Examples:** `Button`, `Table` (basic), `Dialog`, `Badge`, `Tag`, `Separator`, `Kbd`, `Spinner`, `Skeleton`, `GroupBox`, `Breadcrumb`, `DescriptionList`

**Pattern:**
```rust
#[derive(IntoElement)]
pub struct MyComponent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
    // ... configuration fields
}
impl MyComponent { pub fn new(...) -> Self { ... } }
impl ParentElement for MyComponent { ... }
impl Styled for MyComponent { ... }
impl RenderOnce for MyComponent { ... }
```

**When to use:** The component has no internal state that needs to persist across renders. All state is passed in as parameters.

### 2. Stateful Components (Entity + Render)

These components wrap an `Entity<State>` that manages persistent state. The element itself is stateless (RenderOnce), but it reads from a shared Entity.

**Examples:** `Input` (wraps `Entity<InputState>`), `Select` (wraps `Entity<SelectState>`), `DataTable` (wraps `Entity<TableState<D>>`), `ColorPicker` (wraps `Entity<ColorPickerState>`)

**Pattern:**
```rust
pub struct MyComponent {
    state: Entity<MyState>,  // shared state entity
    // ... configuration fields
}
impl MyComponent {
    pub fn new(state: &Entity<MyState>) -> Self { ... }
}
impl RenderOnce for MyComponent {
    fn render(self, window, cx) -> impl IntoElement {
        let state = self.state.read(cx);
        // ... render using state
    }
}
```

**State placement:** The state struct is in the same module or in a separate `state.rs`. The state typically implements `Focusable`, has its own `focus_handle`, and may emit events via `EventEmitter`.

**When to use:** The component manages mutable data (text input, selection, table data) that needs to persist and be shared across renders.

### 3. Composite Components

These build on top of existing components, composing them into higher-level patterns.

**Examples:** `AlertDialog` (built on `Dialog`), `DropdownButton` (built on `Button` + `Menu`), `ButtonGroup` (built on `Button` + layout), `Toggle` (built on `Button`)

**Pattern:** The composite component typically delegates to the base component's API through `WindowExt` methods or wraps them with pre-configured settings.

**When to use:** You need a higher-level pattern that reuses existing component behaviors.

---

## Module Organization Conventions

### Single-file components
- Keep in a single `.rs` file in `crates/ui/src/`
- Used when the component is small and has no sub-elements
- Examples: `badge.rs`, `separator.rs`, `spinner.rs`, `skeleton.rs`, `tag.rs`, `switch.rs`, `slider.rs`, `rating.rs`, `progress.rs`, `pagination.rs`, `link.rs`, `label.rs`, `kbd.rs`, `checkbox.rs`

### Directory-based components
- Create a directory when the component has multiple sub-types or related files
- Use `mod.rs` for re-exports
- Examples: `button/` (button.rs, button_group.rs, button_icon.rs, dropdown_button.rs, toggle.rs), `dialog/` (dialog.rs, header.rs, footer.rs, title.rs, description.rs, content.rs, alert_dialog.rs), `input/` (input.rs, state.rs, number_input.rs, otp_input.rs, element.rs, change.rs, cursor.rs, selection.rs, etc.)

### Story structure
- One story file per component in `crates/story/src/stories/`
- Named `<component>_story.rs` (e.g., `button_story.rs`, `dialog_story.rs`)
- Registration in `crates/story/src/stories/mod.rs`

---

## Event Handling Patterns

### Actions System

The project uses GPUI's `actions!` macro for keyboard-triggered actions:

```rust
actions!(my_namespace, [MyAction, OtherAction]);

// Bind in init
cx.bind_keys([
    KeyBinding::new("escape", MyAction, None),
]);

// Handle in component
.on_action(|_: &MyAction, window, cx| { ... })
```

### Click Events

Buttons and interactive elements use the builder callback pattern:

```rust
Button::new("id")
    .on_click(|_: &ClickEvent, window: &mut Window, cx: &mut App| {
        // handle click
    })
    .on_hover(|hovered: &bool, window: &mut Window, cx: &mut App| {
        // handle hover state change
    })
```

### Using `cx.listener()`

For stateful views (implementing `Render`), use `cx.listener()` to create closures that capture a weak reference to `self`:

```rust
.on_click(cx.listener(|this: &mut MyView, _: &ClickEvent, window, cx| {
    this.some_field = new_value;
    cx.notify();
}))
```

### Mouse Events

Direct mouse event handlers for custom interaction:

```rust
.on_mouse_down(MouseButton::Left, |event, window, cx| {
    window.prevent_default();  // prevent focus on mouse down
    cx.stop_propagation();     // stop event bubbling
})
```

### Keyboard Events

Components register keybindings in their `init()` function and handle via `on_action`:

```rust
.on_action(window.listener_for(&self.state, InputState::backspace))
```

---

## Size System

Components that support sizing implement the `Sizable` trait from `crates/ui/src/styled.rs`:

```rust
pub trait Sizable: Sized {
    fn with_size(self, size: impl Into<Size>) -> Self;
}
```

`Size` enum values: `XSmall` | `Small` | `Medium` (default) | `Large` | `Size(Pixels)` (custom)

Convenience methods: `.xsmall()`, `.small()`, `.large()`

Size affects: padding, font size, height, and spacing uniformly across components.

The `StyleSized` trait provides concrete size mappings:
- `input_size()`, `input_h()`, `input_px()`, `input_py()` — for input-like components
- `list_size()`, `list_px()`, `list_py()` — for list items
- `table_cell_size()` — for table cells
- `button_text_size()` — for button labels
- `size_with()` — generic sizing (h_5/h_6/h_8/h_11 mapping)

---

## Theme System

Located in `crates/ui/src/theme/`. See the theming skill for full details. Key points:

- `Theme` is a Global singleton, accessed via `cx.theme()` (from `ActiveTheme` trait)
- Supports `ThemeMode::Light` and `ThemeMode::Dark`
- Uses `ThemeConfig` (JSON-deserializable) for theme definition
- Colors available as `ThemeColor` struct accessed via `Deref<Target = ThemeColor>` on `Theme`
- Semantic tokens via `ThemeTokens` for component-specific colors
- Supports Oklab color mixing via `Colorize` trait (`mix_oklab()`)

---

## Dock/Panel Layout System

The Dock system (`crates/ui/src/dock/`) provides the complex panel layout:

**Core types:**
- `DockArea` — main container managing center + left/bottom/right docks
- `DockItem` — tree-based structure: `Split`, `Tabs`, `Panel`
- `Panel` trait — defines panel behavior (name, title, close, zoom, toolbar, dump/restore)
- `PanelRegistry` — global registry for panel serialization/deserialization
- `StackPanel` — resizable split panel container
- `TabPanel` — tab panel container

**Panel trait (core methods):**
```rust
pub trait Panel: Focusable + Render + EventEmitter<PanelEvent> {
    fn panel_name(&self) -> &'static str;
    fn title(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
    fn closable(&self, cx: &App) -> bool;
    fn zoomable(&self, cx: &App) -> Option<PanelControl>;
    fn visible(&self, cx: &App) -> bool;
    fn dump(&self, cx: &App) -> PanelState;
}
```

**Register panels:** Call `register_panel(cx, name, constructor_fn)` during app init.

---

## Input System

The input system (`crates/ui/src/input/`) is based on the Rope data structure (from `ropey` crate):

- `InputState` — Entity-based state management (shared between elements)
- `Input` — Element that renders an input field
- `Rope`, `RopeExt`, `RopeLines` — re-exported for external use
- LSP integration (feature-gated with `code-editor` feature)
- Syntax highlighting via Tree-sitter (feature-gated)
- Mask patterns (`MaskPattern`), number input (`NumberInput`), OTP input (`OtpInput`)

**Input variants:**
- `Input::new(&state)` — single-line text input
- `NumberInput::new()` — numeric input
- `OtpInput::new(count)` — one-time password input

---

## Focus System

- `FocusHandle` is used for focus management
- Components track focus with `track_focus(&handle)` or `track_focus(handle.tab_index(n).tab_stop(b))`
- `focus_ring()` method (from `FocusableExt` trait) adds visual focus ring
- `FocusTrapElement` wraps a group of elements to trap Tab cycling within them

---

## Code Style Guidelines

1. **No comments in code** — Do NOT add any comments unless the existing code has them
2. **No emojis** in code or output unless the user explicitly asks
3. **Follow existing patterns** — Match the style of neighboring/related files
4. **Comments on examples** — The `examples/` folder guidelines say to add comments at key parts
5. **PR titles** — Match the existing style in the repo, don't use conventional commits like `fix:` or `feat:` unless the existing style does
6. **Documentation** — When modifying docs, sync both English (`docs/docs/`) and Chinese (`docs/zh-CN/docs/`) versions

## Dependencies & Key Libraries

| Library | Usage |
|---------|-------|
| `ropey` | Rope data structure for text editing |
| `tree-sitter` | Syntax highlighting in code editor |
| `lsp-types` | LSP protocol integration (feat: code-editor) |
| `markdown` | Markdown rendering |
| `html5ever` | HTML rendering |
| `chrono` | Date/time support (feat: time) |
| `rust_decimal` | Decimal number support (feat: decimal) |
| `rust-i18n` | Internationalization (en, zh-CN, zh-HK) |
| `schemars` | JSON Schema generation for theme config |
| `serde` / `serde_json` | Serialization for dock layout, theme config |
