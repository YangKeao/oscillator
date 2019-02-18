use crate::oscillator::Oscillator;
use crate::setting::Settings;
use crate::utils::color::Color;
use std::sync::Arc;

pub struct Bar {
    settings: Arc<Settings>,
    width: u32,
    height: u32,
}

impl Bar {
    pub fn new(settings: Arc<Settings>, width: u32) -> Bar {
        let bar_height = settings.get_bar().height;
        Bar {
            settings,
            width,
            height: bar_height
        }
    }

    pub fn draw(&self, root: &Oscillator) {
        root.fill_rect(0, 0, self.width as i32, self.settings.get_bar().height as i32, Color::from(&self.settings.get_bar().background_color));
    }
}
