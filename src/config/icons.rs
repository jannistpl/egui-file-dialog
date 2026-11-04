/// The tint of an icon.
#[derive(Debug, Clone)]
pub enum IconTint {
    /// The icon is not being tinted.
    None,

    /// The icon is tinted black or white based on the theme.
    /// Dark theme = white, light theme = black.
    BlackWhite,

    /// Use a custom tint.
    Custom(egui::Color32),
}

/// The source of the icon.
#[derive(Debug, Clone)]
pub enum IconSource {
    /// An egui image.
    Image(egui::ImageSource<'static>),

    /// A Unicode character or string.
    Unicode(String),
}

/// An icon used in the user interface of the file dialog.
#[derive(Debug, Clone)]
pub struct Icon {
    source: IconSource,
    tint: IconTint,
}

impl From<String> for Icon {
    fn from(value: String) -> Self {
        Self {
            source: IconSource::Unicode(value),
            tint: IconTint::None,
        }
    }
}

impl From<&str> for Icon {
    fn from(value: &str) -> Self {
        Self {
            source: IconSource::Unicode(value.to_owned()),
            tint: IconTint::None,
        }
    }
}

/// Public methods
impl Icon {
    pub fn image(source: egui::ImageSource<'static>) -> Self {
        Self {
            source: IconSource::Image(source),
            tint: IconTint::None,
        }
    }

    pub fn unicode(text: impl Into<String>) -> Self {
        Self {
            source: IconSource::Unicode(text.into()),
            tint: IconTint::None,
        }
    }

    pub fn tint(mut self, tint: IconTint) -> Self {
        self.tint = tint;
        self
    }
}

/// Crate only methods
impl Icon {
    pub(crate) fn show(&self, ui: &mut egui::Ui) -> egui::Response {
        match &self.source {
            IconSource::Image(source) => {
                let img = self.build_image(ui, source.clone());
                ui.add(img)
            }
            IconSource::Unicode(text) => {
                if let Some(color) = self.get_color(ui) {
                    ui.colored_label(color, text)
                } else {
                    ui.label(text)
                }
            }
        }
    }

    pub(crate) fn into_button<'a>(&'a self, ui: &egui::Ui) -> egui::Button<'a> {
        match &self.source {
            IconSource::Image(source) => {
                let img = self.build_image(ui, source.clone());
                egui::Button::image(img)
            }
            IconSource::Unicode(text) => {
                let mut text = egui::RichText::new(text);

                // TODO: Only color the icon, not the text
                if let Some(color) = self.get_color(ui) {
                    text = text.color(color);
                }

                egui::Button::new(text)
            }
        }
    }

    pub(crate) fn into_button_with_text<'a>(
        &'a self,
        ui: &egui::Ui,
        text: impl AsRef<str>,
    ) -> egui::Button<'a> {
        let text = text.as_ref();

        match &self.source {
            IconSource::Image(source) => {
                let img = self.build_image(ui, source.clone());
                egui::Button::image_and_text(img, text)
            }
            IconSource::Unicode(icon) => {
                let mut text = egui::RichText::new(format!("{icon}  {text}"));

                // TODO: Only color the icon, not the text
                if let Some(color) = self.get_color(ui) {
                    text = text.color(color);
                }

                egui::Button::new(text)
            }
        }
    }

    pub(crate) fn show_as_button_with_text(&self, ui: &mut egui::Ui, text: &str) -> egui::Response {
        let button = self.into_button_with_text(ui, text);
        ui.add(button)
    }

    pub(crate) fn show_as_button_sized(
        &self,
        ui: &mut egui::Ui,
        size: egui::Vec2,
    ) -> egui::Response {
        ui.spacing_mut().button_padding = egui::Vec2::ZERO;
        let button = self.into_button(ui);
        ui.add_sized(size, button)
    }

    fn get_color(&self, ui: &egui::Ui) -> Option<egui::Color32> {
        match self.tint {
            IconTint::None => None,
            IconTint::BlackWhite => {
                if ui.style().visuals.dark_mode {
                    Some(egui::Color32::WHITE)
                } else {
                    Some(egui::Color32::BLACK)
                }
            }
            IconTint::Custom(tint) => Some(tint),
        }
    }

    fn build_image(
        &self,
        ui: &egui::Ui,
        source: egui::ImageSource<'static>,
    ) -> egui::Image<'static> {
        let height = ui.text_style_height(&egui::TextStyle::Body);
        let size = egui::Vec2::new(height, height);

        let mut img = egui::Image::new(source)
            .maintain_aspect_ratio(true)
            .fit_to_exact_size(size);

        if let Some(tint) = self.get_color(ui) {
            img = img.tint(tint);
        }

        img
    }
}
