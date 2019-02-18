use crate::oscillator::Oscillator;
use crate::setting::*;
use crate::utils::color::Color;
use std::collections::HashSet;
use std::sync::Arc;
use std::cell::RefCell;

pub struct Window {
    // INPUT:
    window_id: u32,
    focused: bool,
    tags: HashSet<u32>,

    // OUTPUT:
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    mapped: bool,
    border: u32,
    border_color: Color,
}

pub struct LayoutManager {
    windows: Vec<Window>,
    settings: Arc<Settings>,
    width: u32,
    height: u32,
    current_tag: Arc<RefCell<HashSet<u32>>>,
}

impl LayoutManager {
    pub fn new(settings: Arc<Settings>, width: u32, height: u32, current_tag: Arc<RefCell<HashSet<u32>>>) -> LayoutManager {
        LayoutManager {
            windows: Vec::new(),
            settings,
            width,
            height,
            current_tag,
        }
    }

    pub fn recalc(&mut self) {
        let length = self.windows.len() as u32;

        match self.settings.get_layout_manager_settings() {
            LayoutManagerSettings::Stack {
                border,
                focus_border_color,
                normal_border_color,
            } => {
                if length == 1 {
                    self.windows[0].x = 0;
                    self.windows[0].y = self.settings.get_bar().height;
                    self.windows[0].width = self.width - 2 * *border;
                    self.windows[0].height =
                        self.height - self.settings.get_bar().height - 2 * *border;
                    self.windows[0].mapped = self.current_tag.borrow().intersection(&self.windows[0].tags).into_iter().peekable().peek() != None;
                } else if length > 1 {
                    self.windows[0].x = 0;
                    self.windows[0].y = self.settings.get_bar().height;
                    self.windows[0].width = self.width / 2 - 2 * border;
                    self.windows[0].height =
                        self.height - self.settings.get_bar().height - 2 * border;
                    self.windows[0].mapped = self.current_tag.borrow().intersection(&self.windows[0].tags).into_iter().peekable().peek() != None;
                }

                let item_height = if length > 1 {
                    (self.height - self.settings.get_bar().height) / (length - 1) - 2 * border
                } else {
                    self.height - self.settings.get_bar().height - 2 * border
                };
                for index in 1..(length as usize) {
                    self.windows[index].x = self.width / 2;
                    self.windows[index].y = (item_height + 2 * border) * ((index - 1) as u32)
                        + self.settings.get_bar().height;
                    self.windows[index].height = item_height;
                    self.windows[index].width = self.width / 2 - 2 * border;
                    self.windows[index].mapped = self.current_tag.borrow().intersection(&self.windows[index].tags).into_iter().peekable().peek() != None;
                }

                for window in &mut self.windows {
                    window.border = *border;
                    if window.focused {
                        window.border_color = Color::from(focus_border_color)
                    } else {
                        window.border_color = Color::from(normal_border_color)
                    }
                }
            }
        }
    }

    pub fn sync(&self, root: &Oscillator) {
        for window in &self.windows {
            if window.mapped {
                root.move_and_resize_window(
                    window.window_id,
                    window.x,
                    window.y,
                    window.width,
                    window.height,
                );
                root.set_window_border(window.window_id, window.border, window.border_color);
                root.map_window(window.window_id); // TODO: check tags
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
            focused: false,
            tags,

            width: 0,
            height: 0,
            x: 0,
            y: 0,
            border: 0,
            border_color: Color::new(0, 0, 0, 0),
            mapped: false,
        });
    }

    pub fn focus(&mut self, window_id: u32) {
        for window in &mut self.windows {
            if window.window_id == window_id {
                window.focused = true
            } else {
                window.focused = false
            }
        }
    }

    pub fn unmanage(&mut self, window_id: u32) {
        info!("Unmanage window {}", window_id);
        self.windows
            .retain(|item: &Window| item.window_id != window_id);
    }

    pub fn move_focused_window_to(&mut self, tag: u32) {
        for window in &mut self.windows {
            if window.focused {
                window.tags.clear();
                window.tags.insert(tag);
            }
        }
    }
}
