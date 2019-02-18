use crate::bar::Bar;
use crate::keyboard::keymod::MOD_MAP;
use crate::keyboard::keysymdef::KEYSYM_MAP;
use crate::layout_manager::LayoutManager;
use crate::setting::Key;
use crate::setting::Settings;
use crate::utils::color::Color;
use crate::utils::font::TextExtends;
use image::GenericImageView;
use std::sync::Arc;

pub struct Oscillator {
    connection: Arc<xcb::Connection>,
    screen_num: i32,
    window_id: u32,
    height: i32,
    width: i32,
    settings: Arc<Settings>,
    layout_manager: std::cell::RefCell<LayoutManager>,
    bar: std::cell::RefCell<Bar>,
}

impl Oscillator {
    pub fn setup(settings: Settings) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

        let setup = connection.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root_id = screen.root();
        let width = screen.width_in_pixels() as i32;
        let height = screen.height_in_pixels() as i32;

        let settings = Arc::new(settings);
        let connection = Arc::new(connection);
        let _self = Oscillator {
            connection: connection.clone(),
            screen_num,
            window_id: root_id,
            width,
            height,
            settings: settings.clone(),
            layout_manager: std::cell::RefCell::new(LayoutManager::new(
                settings.clone(),
                width as u32,
                height as u32,
            )),
            bar: std::cell::RefCell::new(Bar::new(settings.clone(), width as u32)),
        };

        const EVENT_MASK: u32 = xcb::EVENT_MASK_KEY_PRESS
            | xcb::EVENT_MASK_BUTTON_PRESS
            | xcb::EVENT_MASK_POINTER_MOTION
            // | xcb::EVENT_MASK_ENTER_WINDOW
            | xcb::EVENT_MASK_LEAVE_WINDOW
            | xcb::EVENT_MASK_STRUCTURE_NOTIFY
            | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY
            | xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT;
        xcb::map_window(&_self.connection, _self.window_id);

        let title = "Oscillator";
        xcb::change_property(
            &_self.connection,
            xcb::PROP_MODE_REPLACE as u8,
            _self.window_id,
            xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING,
            8,
            title.as_bytes(),
        );

        let cursor_font = _self.connection.generate_id();
        xcb::open_font(&_self.connection, cursor_font, "cursor");
        let cursor = _self.connection.generate_id();
        xcb::create_glyph_cursor(
            &_self.connection,
            cursor,
            cursor_font,
            cursor_font,
            68,
            68 + 1,
            0,
            0,
            0,
            0,
            0,
            0,
        );
        xcb::change_window_attributes(
            &_self.connection,
            _self.window_id,
            &[(xcb::CW_EVENT_MASK, EVENT_MASK), (xcb::CW_CURSOR, cursor)],
        );
        info!(
            "Setup root window. Width: {}, Height: {}",
            _self.width, _self.height
        );

        //        _self.set_background(settings.get_background());

        _self.focus(_self.window_id);
        _self.bar.borrow().draw(&_self);

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

                            let mut key_string: String = format!("{}", KEYSYM_MAP[&keysym]);
                            for i in 0..8 {
                                if keymod & 1 << (7 - i) > 0 {
                                    key_string =
                                        format!("{}-{}", MOD_MAP[&(1 << (7 - i))], key_string);
                                }
                            }
                            info!("Trigger {}", key_string);

                            match &self.settings.get_keys().get(&key_string.to_lowercase()) {
                                Some(Key::Spawn { command }) => {
                                    info!("Spawn: \"{}\"", command.join(" "));
                                    let mut proc = std::process::Command::new(&command[0]);
                                    for arg in 1..command.len() {
                                        proc.arg(&command[arg]);
                                    }
                                    match proc.spawn() {
                                        Err(_) => {
                                            info!("Spawn Failed"); //TODO: More detailed spawn failed information
                                        }
                                        _ => info!("Spawn \"{}\" successful", command.join(" ")),
                                    }
                                }
                                Some(Key::Quit) => {
                                    info!("Quit focus window");

                                    let window = xcb::get_input_focus(&self.connection)
                                        .get_reply()
                                        .unwrap()
                                        .focus();
                                    xcb::kill_client(&self.connection, window);
                                    self.flush();
                                }
                                None => {}
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
                            let enter_notify_event: &xcb::EnterNotifyEvent =
                                unsafe { xcb::cast_event(&event) };

                            self.focus(enter_notify_event.event());
                            self.layout_manager.borrow().sync(self);
                            self.flush();

                            trace!(
                                "Event ENTER_NOTIFY triggered on WINDOW: {}",
                                enter_notify_event.event()
                            );
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
                            self.layout_manager.borrow_mut().manage(window);
                            self.listen_window_event(window);
                            self.layout_manager.borrow().sync(self);

                            trace!("Event MAP_REQUEST triggered");
                        }
                        xcb::UNMAP_NOTIFY => {
                            trace!("Event UNMAP_NOTIFY triggered");
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
                            let destroy_notify_event: &xcb::DestroyNotifyEvent =
                                unsafe { xcb::cast_event(&event) };

                            let window = destroy_notify_event.window();
                            self.layout_manager.borrow_mut().unmanage(window);
                            self.layout_manager.borrow().sync(self);
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
                        xcb::PROPERTY_NOTIFY => {
                            trace!("Event PROPERTY_NOTIFY triggered");
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
        let screen = self
            .connection
            .get_setup()
            .roots()
            .nth(self.screen_num as usize)
            .unwrap();
        let foreground = self.connection.generate_id();

        xcb::create_gc(
            &self.connection,
            foreground,
            screen.root(),
            &[
                (xcb::GC_FOREGROUND, color.into()),
                (xcb::GC_GRAPHICS_EXPOSURES, 0),
            ],
        );
        xcb::poly_fill_rectangle(
            &self.connection,
            self.window_id,
            foreground,
            &[xcb::Rectangle::new(x as i16, y as i16, w as u16, h as u16)],
        );
    }

    pub fn focus(&self, window: u32) {
        info!("Focus on WINDOW {}", window);

        self.layout_manager.borrow_mut().focus(window);
        xcb::set_input_focus(&self.connection, 1, window, xcb::CURRENT_TIME);
    }

    pub fn set_window_border(&self, window: u32, border_width: u32, border_color: Color) {
        xcb::configure_window(
            &self.connection,
            window,
            &[(xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, border_width)],
        );
        xcb::change_window_attributes(
            &self.connection,
            window,
            &[(xcb::CW_BORDER_PIXEL, border_color.into())],
        );

        println!(
            "{:#X} {:#X} {:#X} {:#X}",
            border_color.r, border_color.g, border_color.b, border_color.a
        );
    }

    pub fn listen_window_event(&self, window: u32) {
        const EVENT_MASK: u32 = xcb::EVENT_MASK_KEY_PRESS
            | xcb::EVENT_MASK_ENTER_WINDOW
            | xcb::EVENT_MASK_STRUCTURE_NOTIFY;

        xcb::change_window_attributes(
            &self.connection,
            window,
            &[(xcb::CW_EVENT_MASK, EVENT_MASK)],
        );
    }

    pub fn unmap_window(&self, window: u32) {
        info!("Unmap window {}", window);
        xcb::unmap_window(&self.connection, window);
    }

    pub fn map_window(&self, window: u32) {
        info!("Map window {}", window);
        xcb::map_window(&self.connection, window);
    }

    pub fn resize_window(&self, window: u32, width: u32, height: u32) {
        info!(
            "Resize window {} into WIDTH: {} HEIGHT: {}",
            window, width, height
        );
        xcb::configure_window(
            &self.connection,
            window,
            &[
                (xcb::CONFIG_WINDOW_WIDTH as u16, width),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
            ],
        );
    }

    pub fn move_window(&self, window: u32, x: u32, y: u32) {
        info!("Move window {} into X: {} Y: {}", window, x, y);
        xcb::configure_window(
            &self.connection,
            window,
            &[
                (xcb::CONFIG_WINDOW_X as u16, x),
                (xcb::CONFIG_WINDOW_Y as u16, y),
            ],
        );
    }

    pub fn move_and_resize_window(&self, window: u32, x: u32, y: u32, width: u32, height: u32) {
        info!(
            "Move and Resize window {} into X: {} Y: {} WIDTH: {} HEIGHT: {}",
            window, x, y, width, height
        );
        xcb::configure_window(
            &self.connection,
            window,
            &[
                (xcb::CONFIG_WINDOW_X as u16, x),
                (xcb::CONFIG_WINDOW_Y as u16, y),
                (xcb::CONFIG_WINDOW_WIDTH as u16, width),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
            ],
        );
    }

    pub fn set_background(&self, background_src: &str) {
        unimplemented!();
        /*
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
        */
    }

    pub fn query_text_extents(&self, font_name: &str, s: &str) -> TextExtends {
        let font = self.connection.generate_id();
        xcb::open_font(&self.connection, font, font_name);

        let len = s.len();
        let ptr = s.as_ptr();
        let ptr = ptr as *const xcb::Char2b;
        let text_extends = xcb::query_text_extents(&self.connection, font, unsafe {std::slice::from_raw_parts(ptr, len)}).get_reply().unwrap();
        return TextExtends {
            overall_width: text_extends.overall_width(),
            font_ascent: text_extends.font_ascent(),
            font_descent: text_extends.font_descent(),
        }
    }

    pub fn flush(&self) {
        self.connection.flush();
    }
}
