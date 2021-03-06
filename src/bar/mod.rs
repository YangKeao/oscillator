use crate::oscillator::Oscillator;
use crate::setting::Settings;
use crate::utils::color::Color;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

pub struct Bar {
    settings: Arc<Settings>,
    width: u32,
    height: u32,
    font: u32,
    current_tag: Arc<RefCell<HashSet<u32>>>,
}

impl Bar {
    pub fn new(
        settings: Arc<Settings>,
        width: u32,
        current_tag: Arc<RefCell<HashSet<u32>>>,
    ) -> Bar {
        let bar_height = settings.get_bar().height;
        Bar {
            settings,
            width,
            height: bar_height,
            font: 0,
            current_tag,
        }
    }

    pub fn prepare(&mut self, root: &Oscillator) {
        self.font = root.create_font(&self.settings.get_bar().font_family);
    }

    pub fn draw(&self, root: &Oscillator) {
        root.fill_rect(
            0,
            0,
            self.width as i32,
            self.height as i32,
            Color::from(&self.settings.get_bar().background_color),
        );

        let cell_width = self.settings.get_bar().tag_cell_width;
        // TODO: tags range in settings
        for i in 0..10 {
            let s = format!("{}", i);
            let text_extends = root.query_text_extents(self.font, &s);

            let x = i * cell_width + (cell_width - text_extends.overall_width as u32) / 2;
            let y = self.height / 2
                + ((text_extends.font_ascent + text_extends.font_descent) / 2
                    - text_extends.font_descent) as u32;

            if self.current_tag.borrow().contains(&i) {
                root.fill_rect(
                    (i * cell_width) as i32,
                    0,
                    cell_width as i32,
                    self.height as i32,
                    Color::from(&self.settings.get_bar().active_background_color),
                );
                root.draw_text(
                    x as i32,
                    y as i32,
                    Color::from(&self.settings.get_bar().active_font_color),
                    Color::from(&self.settings.get_bar().active_background_color),
                    self.font,
                    &s,
                );
            } else {
                root.draw_text(
                    x as i32,
                    y as i32,
                    Color::from(&self.settings.get_bar().font_color),
                    Color::from(&self.settings.get_bar().background_color),
                    self.font,
                    &s,
                );
            }
        }
    }
}
