use std::sync::Arc;
use crate::setting::Settings;
use crate::setting::Key;
use crate::keyboard::keysymdef::KEYSYM_MAP;
use crate::keyboard::keymod::MOD_MAP;
use image::GenericImageView;

pub struct Oscillator {
    connection: Arc<xcb::Connection>,
    screen_num: i32,
    window_id: u32,
    height: i32,
    width: i32,
    settings: Settings,
}

pub struct Color {}

impl Default for Color {
    fn default() -> Self {
        Color {}
    }
}

impl Oscillator {
    pub fn setup(settings: Settings) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

        let setup = connection.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root_id = screen.root();
        let width = screen.width_in_pixels() as i32;
        let height = screen.height_in_pixels() as i32;

        let _self = Oscillator {
            connection: Arc::new(connection),
            screen_num,
            window_id: root_id,
            width,
            height,
            settings
        };

        const EVENT_MASK: u32 = xcb::EVENT_MASK_KEY_PRESS
            | xcb::EVENT_MASK_BUTTON_PRESS
            | xcb::EVENT_MASK_POINTER_MOTION
            | xcb::EVENT_MASK_BUTTON_MOTION
            | xcb::EVENT_MASK_BUTTON_1_MOTION
            | xcb::EVENT_MASK_BUTTON_2_MOTION
            | xcb::EVENT_MASK_BUTTON_3_MOTION
            | xcb::EVENT_MASK_BUTTON_4_MOTION
            | xcb::EVENT_MASK_BUTTON_5_MOTION
            | xcb::EVENT_MASK_ENTER_WINDOW
            | xcb::EVENT_MASK_LEAVE_WINDOW
            | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY
            | xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT;
        xcb::map_window(&_self.connection, _self.window_id);

        let title = "Oscillator";
        xcb::change_property(&_self.connection, xcb::PROP_MODE_REPLACE as u8, _self.window_id,
                             xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());

        let cursor_font = _self.connection.generate_id();
        xcb::open_font(&_self.connection, cursor_font, "cursor");
        let cursor = _self.connection.generate_id();
        xcb::create_glyph_cursor(&_self.connection, cursor,
                                 cursor_font, cursor_font,
                                 68, 68 + 1, 0, 0, 0,
                                 0, 0, 0);
        xcb::change_window_attributes(&_self.connection, _self.window_id, &[
            (xcb::CW_EVENT_MASK, EVENT_MASK),
            (xcb::CW_CURSOR, cursor)
        ]);
        info!("Setup root window. Width: {}, Height: {}", _self.width, _self.height);

//        _self.set_background(settings.get_background());

        _self.flush();

        return _self;
    }

    pub fn main_loop(&self) {
        loop {
            let event = self.connection.wait_for_event();

            match event {
                None => {
                    warn!("IO Error");
                }
                Some(event) => {
                    let r = event.response_type() & !0x80;
                    match r {
                        xcb::KEY_PRESS => {
                            let key_press_event: &xcb::KeyPressEvent =
                                unsafe { xcb::cast_event(&event) };

                            let key_symbols = xcb_util::keysyms::KeySymbols::new(&self.connection);
                            let keysym = key_symbols.press_lookup_keysym(key_press_event, 0); //TODO: what is col?
                            let keymod = key_press_event.state();

                            let mut key_string = format!("{}", KEYSYM_MAP[&keysym]);
                            for i in 0..8 {
                                if keymod & 1 << (7-i) > 0 {
                                    key_string = format!("{}-{}", MOD_MAP[&(1 << (7-i))], key_string);
                                }
                            }
                            info!("Trigger {}", key_string);

                            match &self.settings.get_keys().get(&key_string) {
                                Some(Key::Spawn{ command }) => {
                                    info!("Spawn: \"{}\"", command.join(" "));
                                    let mut proc = std::process::Command::new(&command[0]);
                                    for arg in 1..command.len() {
                                        proc.arg(&command[arg]);
                                    }
                                    match proc.spawn() {
                                        Err(_) => {
                                            info!("Spawn Failed"); //TODO: More detailed spawn failed information
                                        }
                                        _ => {
                                            info!("Spawn \"{}\" successful", command.join(" "))
                                        }
                                    }
                                    println!("RUNRUNRUN");
                                }
                                None => {
                                }
                            }
                            trace!(
                                "Event KEY_PRESS triggered on WINDOW: {}",
                                key_press_event.event()
                            );
                        }
                        xcb::BUTTON_PRESS => {
                            let button_press_event: &xcb::ButtonPressEvent =
                                unsafe { xcb::cast_event(&event) };
                            trace!(
                                "Event BUTTON_PRESS triggered on WINDOW: {}",
                                button_press_event.event()
                            );
                        }
                        xcb::MOTION_NOTIFY => {
                            trace!("Event MOTION_NOTIFY triggered");
                        }
                        xcb::ENTER_NOTIFY => {
                            trace!("Event ENTER_NOTIFY triggered");
                        }
                        xcb::LEAVE_NOTIFY => {
                            trace!("Event LEAVE_NOTIFY triggered");
                        }
                        xcb::CLIENT_MESSAGE => {
                            trace!("Event CLIENT_MESSAGE triggered");
                        }
                        xcb::MAP_REQUEST => {
                            let map_request_event: &xcb::MapRequestEvent =
                                unsafe { xcb::cast_event(&event) };

                            let window = map_request_event.window();
                            self.map_window(window);
                            self.move_and_resize_window(window, 0, 0, 600, 400);

                            self.flush();
                            trace!("Event MAP_REQUEST triggered");
                        }
                        xcb::CIRCULATE_REQUEST => {
                            trace!("Event CIRCULATE_REQUEST triggered");
                        }
                        xcb::CONFIGURE_REQUEST => {
                            trace!("Event CONFIGURE_REQUEST triggered");
                        }
                        xcb::CIRCULATE_NOTIFY => {
                            trace!("Event CIRCULATE_NOTIFY triggered");
                        }
                        xcb::CONFIGURE_NOTIFY => {
                            trace!("Event CONFIGURE_NOTIFY triggered");
                        }
                        xcb::CREATE_NOTIFY => {
                            trace!("Event CREATE_NOTIFY triggered");
                        }
                        xcb::DESTROY_NOTIFY => {
                            trace!("Event DESTROY_NOTIFY triggered");
                        }
                        xcb::GRAVITY_NOTIFY => {
                            trace!("Event GRAVITY_NOTIFY triggered");
                        }
                        xcb::MAP_NOTIFY => {
                            trace!("Event MAP_NOTIFY triggered");
                        }
                        xcb::REPARENT_NOTIFY => {
                            trace!("Event REPARENT_NOTIFY triggered");
                        }
                        xcb::UNMAP_NOTIFY => {
                            trace!("Event UNMAP_NOTIFY triggered");
                        }
                        0 => {
                            let error_message: &xcb::GenericError =
                                unsafe { xcb::cast_event(&event) };
                            warn!(
                                "XCB Error Code: {}, Major Code: {}, Minor Code: {}",
                                error_message.error_code(),
                                unsafe { (*error_message.ptr).major_code },
                                unsafe { (*error_message.ptr).minor_code }
                            );
                        }
                        _ => {
                            warn!("Unhandled Event");
                        }
                    }
                }
            }
        }
    }

    pub fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        let screen = self.connection.get_setup().roots().nth(self.screen_num as usize).unwrap();
        let foreground = self.connection.generate_id();

        xcb::create_gc(&self.connection, foreground, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.white_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);
        xcb::poly_fill_rectangle(&self.connection, self.window_id, foreground, &[
            xcb::Rectangle::new(x as i16, y as i16, w as u16, h as u16)
        ]);
    }

    pub fn map_window(&self, window: u32) {
        info!("Map window {}", window);
        xcb::map_window(&self.connection, window);
    }

    pub fn resize_window(&self, window: u32, width: u32, height: u32) {
        info!("Resize window {} into WIDTH: {} HEIGHT: {}", window, width, height);
        xcb::configure_window(&self.connection, window, &[
            (xcb::CONFIG_WINDOW_WIDTH as u16, width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
        ]);
    }

    pub fn move_window(&self, window: u32, x: u32, y: u32) {
        info!("Move window {} into X: {} Y: {}", window, x, y);
        xcb::configure_window(&self.connection, window, &[
            (xcb::CONFIG_WINDOW_X as u16, x),
            (xcb::CONFIG_WINDOW_Y as u16, y),
        ]);
    }

    pub fn move_and_resize_window(&self, window: u32, x: u32, y: u32, width: u32, height: u32) {
        info!("Move and Resize window {} into X: {} Y: {} WIDTH: {} HEIGHT: {}", window, x, y, width, height);
        xcb::configure_window(&self.connection, window, &[
            (xcb::CONFIG_WINDOW_X as u16, x),
            (xcb::CONFIG_WINDOW_Y as u16, y),
            (xcb::CONFIG_WINDOW_WIDTH as u16, width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
        ]);
    }

    pub fn set_background(&self, background_src: &str) {
        let screen = self.connection.get_setup().roots().nth(self.screen_num as usize).unwrap();
        let foreground = self.connection.generate_id();
        xcb::create_gc(&self.connection, foreground, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.white_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);

        info!("Set background {}", background_src);
        let mut img = image::open(background_src).unwrap();
        let img_width = img.width();
        let img_height = img.height();
        let mut img_buffer = img.to_rgb();
        info!("Background WIDTH: {} HEIGHT: {}", img_width, img_height);

        let pixmap = self.connection.generate_id();
        xcb::create_pixmap(&self.connection, 24, pixmap, self.window_id, img_width as u16, img_height as u16);
        // TODO: Set background

        let prop_root = xcb::intern_atom(&self.connection, false, "_XROOTPMAP_ID").get_reply().unwrap().atom();
        let prop_esetroot = xcb::intern_atom(&self.connection, false, "ESETROOT_PMAP_ID").get_reply().unwrap().atom();
        xcb::change_property(&self.connection, xcb::PROP_MODE_REPLACE as u8, self.window_id,
                             prop_root, xcb::ATOM_PIXMAP, 32, &[pixmap]);
        xcb::change_property(&self.connection, xcb::PROP_MODE_REPLACE as u8, self.window_id,
                             prop_esetroot, xcb::ATOM_PIXMAP, 32, &[pixmap]);

        xcb::change_window_attributes(&self.connection, self.window_id, &[
            (xcb::CW_BACK_PIXMAP, pixmap),
        ]);
    }

    pub fn flush(&self) {
        self.connection.flush();
    }
}
