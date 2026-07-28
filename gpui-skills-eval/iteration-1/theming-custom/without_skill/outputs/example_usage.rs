// Example: How to use `apply_midnight_purple_theme` in a GPUI application.
//
// Paste this into your app's entry point after `gpui_component::init(cx)`.
//
// ```rust
// use gpui_component_story::apply_midnight_purple_theme;
//
// fn main() {
//     let app = gpui_platform::application();
//     app.run(move |cx| {
//         gpui_component::init(cx);
//
//         // Apply the Midnight Purple dark theme
//         apply_midnight_purple_theme(cx);
//
//         cx.spawn(async move |cx| {
//             cx.open_window(WindowOptions::default(), |window, cx| {
//                 let view = cx.new(|_| MyView);
//                 cx.new(|cx| Root::new(view, window, cx))
//             })
//             .expect("Failed to open window");
//         }).detach();
//     });
// }
// ```
