/// Sets the cursor position to the end of a text input field.
///
/// # Arguments
///
/// * `re` - response of the text input widget
/// * `data` - buffer holding the text of the input widget
pub fn set_cursor_to_end(re: &egui::Response, data: &str) {
    // Set the cursor to the end of the filter input string
    if let Some(mut state) = egui::TextEdit::load_state(&re.ctx, re.id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::one(
                egui::text::CCursor::new(data.len()),
            )));

        state.store(&re.ctx, re.id);
    }
}

/// Calculates the width of a single char.
pub fn calc_char_width(ui: &egui::Ui, char: char) -> f32 {
    ui.fonts_mut(|f| f.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), char))
}

/// Calculates the width of the specified text using the current font configuration.
/// Does not take new lines or text breaks into account!
pub fn calc_text_width(ui: &egui::Ui, text: &str) -> f32 {
    let mut width = 0.0;

    for char in text.chars() {
        width += calc_char_width(ui, char);
    }

    width
}
