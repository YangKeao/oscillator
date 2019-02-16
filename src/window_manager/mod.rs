use std::sync::Arc;
use std::collections::HashSet;
use crate::oscillator::Oscillator;
use crate::setting::Settings;

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
    settings: Arc<Settings>
}

impl WindowManager {
    pub fn new(settings: Arc<Settings>, width: u32, height: u32) -> WindowManager {
        WindowManager {
            windows: Vec::new(),
            settings
        }
    }

    fn recalc(&mut self) {
        for index in 0..self.windows.len() {
            // TODO: check tags

        }
    }

    pub fn sync(&self, root: &Oscillator) {
        for window in &self.windows {
            root.unmap_window(window.window_id);
        }
        for window in &self.windows {
            root.move_and_resize_window(window.window_id, window.x, window.y, window.width, window.height);
            root.map_window(window.window_id); // TODO: check tags
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
