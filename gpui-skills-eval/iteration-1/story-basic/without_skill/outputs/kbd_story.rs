use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, Keystroke, ParentElement, Render,
    Styled, Window, div,
};

use gpui_component::{h_flex, kbd::Kbd, v_flex};

use crate::section;

pub struct KbdStory {
    focus_handle: gpui::FocusHandle,
}

impl super::Story for KbdStory {
    fn title() -> &'static str {
        "Kbd"
    }

    fn description() -> &'static str {
        "Keyboard shortcut text display using <kbd> tag style."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl KbdStory {
    pub(crate) fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}
impl Focusable for KbdStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for KbdStory {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                section("Common Shortcuts")
                    .max_w_lg()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Kbd::new(Keystroke::parse("cmd-c").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-v").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-x").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-a").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-z").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-s").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-f").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-p").unwrap())),
                    ),
            )
            .child(
                section("Modifier Combinations")
                    .max_w_lg()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Kbd::new(Keystroke::parse("cmd-shift-p").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-ctrl-t").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-ctrl-shift-a").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-alt-esc").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-ctrl-alt-shift-a").unwrap())),
                    ),
            )
            .child(
                section("Platform Modifier Keys")
                    .sub_title(div().text_color(gpui_component::muted_foreground()).text_sm().child(
                        "Keyboards shortcuts render with platform-specific modifier symbols (⌃⌥⇧⌘ on macOS, Ctrl+Alt+Shift+Win on Windows).",
                    ))
                    .max_w_lg()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Kbd::new(Keystroke::parse("ctrl-a").unwrap()))
                            .child(Kbd::new(Keystroke::parse("alt-a").unwrap()))
                            .child(Kbd::new(Keystroke::parse("shift-a").unwrap()))
                            .child(Kbd::new(Keystroke::parse("cmd-a").unwrap()))
                            .child(Kbd::new(Keystroke::parse("ctrl-alt-shift-win-a").unwrap())),
                    ),
            )
            .child(
                section("Special Keys")
                    .max_w_lg()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Kbd::new(Keystroke::parse("escape").unwrap()))
                            .child(Kbd::new(Keystroke::parse("enter").unwrap()))
                            .child(Kbd::new(Keystroke::parse("backspace").unwrap()))
                            .child(Kbd::new(Keystroke::parse("delete").unwrap()))
                            .child(Kbd::new(Keystroke::parse("tab").unwrap()))
                            .child(Kbd::new(Keystroke::parse("space").unwrap()))
                            .child(Kbd::new(Keystroke::parse("pagedown").unwrap()))
                            .child(Kbd::new(Keystroke::parse("pageup").unwrap()))
                            .child(Kbd::new(Keystroke::parse("left").unwrap()))
                            .child(Kbd::new(Keystroke::parse("right").unwrap()))
                            .child(Kbd::new(Keystroke::parse("up").unwrap()))
                            .child(Kbd::new(Keystroke::parse("down").unwrap())),
                    ),
            )
            .child(
                section("Outline Style")
                    .sub_title(div().text_color(gpui_component::muted_foreground()).text_sm().child(
                        "Outline variant uses border styling instead of filled background.",
                    ))
                    .max_w_lg()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Kbd::new(Keystroke::parse("cmd-shift-p").unwrap()).outline())
                            .child(Kbd::new(Keystroke::parse("cmd-ctrl-t").unwrap()).outline())
                            .child(Kbd::new(Keystroke::parse("enter").unwrap()).outline())
                            .child(Kbd::new(Keystroke::parse("escape").unwrap()).outline())
                            .child(Kbd::new(Keystroke::parse("ctrl-alt-delete").unwrap()).outline()),
                    ),
            )
            .child(
                section("Without Appearance")
                    .sub_title(div().text_color(gpui_component::muted_foreground()).text_sm().child(
                        "Set appearance to false to render plain text without kbd styling.",
                    ))
                    .max_w_lg()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Kbd::new(Keystroke::parse("cmd-c").unwrap()).appearance(false))
                            .child(Kbd::new(Keystroke::parse("cmd-v").unwrap()).appearance(false))
                            .child(Kbd::new(Keystroke::parse("cmd-shift-p").unwrap()).appearance(false)),
                    ),
            )
    }
}
