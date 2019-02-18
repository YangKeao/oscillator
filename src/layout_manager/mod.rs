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
    mapped: bool,
}

pub struct LayoutManager {
    windows: Vec<Window>,
    settings: Arc<Settings>,
    width: u32,
    height: u32,
}

impl LayoutManager {
    pub fn new(settings: Arc<Settings>, width: u32, height: u32) -> LayoutManager {
        LayoutManager {
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
                if length == 1 {
                    self.windows[0].x = 0;
                    self.windows[0].y = 0;
                    self.windows[0].width = self.width;
                    self.windows[0].height = self.height;
                    self.windows[0].mapped = true;
                } else if length > 1 {
                    self.windows[0].x = 0;
                    self.windows[0].y = 0;
                    self.windows[0].width = self.width / 2;
                    self.windows[0].height = self.height;
                    self.windows[0].mapped = true;
                }

                for index in 1..(length as usize) {
                    self.windows[index].x = self.width / 2;
                    self.windows[index].y = self.height / (length - 1) * ((index - 1) as u32);
                    self.windows[index].width = self.width / 2;
                    self.windows[index].height = self.height / (length - 1);
                    self.windows[index].mapped = true;
                }
            }
        }
    }

    pub fn sync(&self, root: &Oscillator) {
        for window in &self.windows {
            if window.mapped {
                root.move_and_resize_window(window.window_id, window.x, window.y, window.width, window.height);
                root.map_window(window.window_id); // TODO: check tags
                root.set_windows_input(window.window_id);
            } else {
                root.unmap_window(window.window_id);
            }
        }
        root.flush();
    }

    pub fn manage(&mut self, window_id: u32) {
        info!("Manage window {}", window_id);
        let mut tags = HashSet::new();
        tags.insert(0);
        self.windows.push(Window {
            window_id,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            tags,
            mapped: false
        });
        self.recalc();
    }

    pub fn unmanage(&mut self, window_id: u32) {
        info!("Unmanage window {}", window_id);
        self.windows.retain(|item: &Window| item.window_id != window_id);
        self.recalc();
    }
}
