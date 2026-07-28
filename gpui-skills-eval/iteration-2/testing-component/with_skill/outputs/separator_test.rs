#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Axis, red};

    #[gpui::test]
    fn test_separator_builder(_cx: &mut gpui::TestAppContext) {
        let separator = Separator::horizontal()
            .label("Section")
            .color(red())
            .dashed();

        assert_eq!(separator.axis, Axis::Horizontal);
        assert_eq!(separator.label, Some("Section".into()));
        assert_eq!(separator.color, Some(red()));
        assert_eq!(separator.line_style, SeparatorStyle::Dashed);
    }

    #[gpui::test]
    fn test_separator_struct_fields(_cx: &mut gpui::TestAppContext) {
        let horizontal = Separator::horizontal();
        assert_eq!(horizontal.axis, Axis::Horizontal);
        assert_eq!(horizontal.label, None);
        assert_eq!(horizontal.color, None);
        assert_eq!(horizontal.line_style, SeparatorStyle::Solid);

        let vertical = Separator::vertical();
        assert_eq!(vertical.axis, Axis::Vertical);
        assert_eq!(vertical.label, None);
        assert_eq!(vertical.color, None);
        assert_eq!(vertical.line_style, SeparatorStyle::Solid);

        let dashed = Separator::horizontal().dashed();
        assert_eq!(dashed.line_style, SeparatorStyle::Dashed);
    }
}
