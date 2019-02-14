use std::sync::Arc;

pub struct X11 {
    connection: Arc<xcb::Connection>,
    screen_num: i32,
    window_id: u32,
}

pub struct Color {}

impl Default for Color {
    fn default() -> Self {
        Color {}
    }
}

impl X11 {
    pub fn new() -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

        let mut x11 = X11 {
            connection: Arc::new(connection),
            screen_num,
            window_id: 0,
        };

        let setup = x11.connection.get_setup();
        let screen = setup.roots().nth(x11.screen_num as usize).unwrap();

        x11.window_id = screen.root();
        let mut width = screen.width_in_pixels() as i32;
        let mut height = screen.height_in_pixels() as i32;

        let event_mask = xcb::EVENT_MASK_EXPOSURE
                    | xcb::EVENT_MASK_KEY_PRESS
                    | xcb::EVENT_MASK_KEY_RELEASE
                    | xcb::EVENT_MASK_BUTTON_PRESS
                    | xcb::EVENT_MASK_BUTTON_RELEASE
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
        xcb::map_window(&x11.connection, x11.window_id);

        let title = "Oscillator";
        xcb::change_property(&x11.connection, xcb::PROP_MODE_REPLACE as u8, x11.window_id,
                             xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());
        xcb::change_window_attributes(&x11.connection, x11.window_id, &[
            (xcb::CW_EVENT_MASK, event_mask)
        ]);
        info!("Create window. Width: {}, Height: {}", width, height);

        x11.fill_rect(0, 0, width, height, Color::default());
        x11.connection.flush();

        return x11;
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
                        xcb::EXPOSE => {
                            let expose_event: &xcb::ExposeEvent =
                                unsafe { xcb::cast_event(&event) };
                            trace!("Event EXPOSE triggered");
                        }
                        xcb::KEY_PRESS => {
                            let key_press_event: &xcb::KeyPressEvent =
                                unsafe { xcb::cast_event(&event) };
                            trace!(
                                "Event KEY_PRESS triggered on WINDOW: {}",
                                key_press_event.event()
                            );
                        }
                        xcb::KEY_RELEASE => {
                            trace!("Event KEY_RELEASE triggered");
                        }
                        xcb::BUTTON_PRESS => {
                            let button_press_event: &xcb::ButtonPressEvent =
                                unsafe { xcb::cast_event(&event) };
                            trace!(
                                "Event BUTTON_PRESS triggered on WINDOW: {}",
                                button_press_event.event()
                            );
                        }
                        xcb::BUTTON_RELEASE => {
                            trace!("Event BUTTON_RELEASE triggered");
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
                            trace!("Event MAP_REQUEST triggered");
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

        self.connection.flush();
    }
}