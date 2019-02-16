use std::sync::Arc;
use std::collections::HashSet;
use crate::oscillator::Oscillator;
use crate::setting::*;

pub struct Window {
    window_id: u32,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    tags: HashSet<u32>,
}

pub struct WindowManager {
    windows: Vec<Window>,
    settings: Arc<Settings>,
    width: u32,
    height: u32,
}

impl WindowManager {
    pub fn new(settings: Arc<Settings>, width: u32, height: u32) -> WindowManager {
        WindowManager {
            windows: Vec::new(),
            settings,
            width,
            height,
        }
    }

    fn recalc(&mut self) {
        let length = self.windows.len() as u32;

        match self.settings.get_tiling_method() {
            TilingMethod::Stack => {
                self.windows[0].x = 0;
                self.windows[0].y = 0;
                self.windows[0].width = self.width / 2; // TODO: if only one window.
                self.windows[0].height = self.height;

                for index in 1..(length as usize) {
                    self.windows[index].x = self.width / 2;
                    self.windows[index].y = self.height / (length - 1) * ((index - 1) as u32);
                    self.windows[index].width = self.width / 2;
                    self.windows[index].height = self.height / (length - 1);
                }
            }
        }
    }

    pub fn sync(&self, root: &Oscillator) {
        for window in &self.windows {
            root.unmap_window(window.window_id);
        }
        for window in &self.windows {
            root.move_and_resize_window(window.window_id, window.x, window.y, window.width, window.height);
            root.map_window(window.window_id); // TODO: check tags
            root.set_windows_input(window.window_id);
        }

        root.flush();
    }

    pub fn manage(&mut self, window_id: u32) {
        self.windows.push(Window {
            window_id,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            tags: HashSet::new()
        });
        self.recalc();
    }
}
