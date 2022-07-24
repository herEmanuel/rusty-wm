use x11rb::{
    rust_connection::RustConnection, 
    protocol::{xproto::*, Event}, 
    connection::Connection, COPY_FROM_PARENT
};

pub struct WindowManager<'s> {
    conn: &'s RustConnection,
    screen: &'s Screen,
}

impl<'s> WindowManager<'s> {
    pub fn new(conn: &'s RustConnection, screen: &'s Screen) -> Self {
        change_window_attributes(conn, screen.root, 
            &ChangeWindowAttributesAux::new()
            .event_mask(EventMask::SUBSTRUCTURE_NOTIFY | EventMask::SUBSTRUCTURE_REDIRECT)).unwrap();

        conn.flush().unwrap();

        println!("we are now the window manager!!! yeeey"); 
        WindowManager { conn, screen }
    }

    pub fn run(&self) {
        loop {
            if let Ok(event) = self.conn.wait_for_event() {
                match event {
                    Event::ConfigureRequest(config_request) => {
                        self.on_configure_request(config_request)
                    }

                    Event::MapRequest(map_request) => {
                        self.on_map_request(map_request);
                    }

                    _ => {
                        println!("{:?}", event);
                    }
                }
            }
        }
    }

    pub fn on_configure_request(&self, config_request: ConfigureRequestEvent) {
        configure_window(self.conn, config_request.window, &ConfigureWindowAux::from_configure_request(&config_request)).unwrap();
    }

    pub fn on_map_request(&self, map_request: MapRequestEvent) {
        let frame_id = self.conn.generate_id().unwrap();
        let window_attr = get_geometry(self.conn, map_request.window).unwrap().reply().unwrap();

        create_window(self.conn, COPY_FROM_PARENT as u8, frame_id, self.screen.root, window_attr.x, window_attr.y, window_attr.width, window_attr.height, 2, WindowClass::INPUT_OUTPUT, COPY_FROM_PARENT, &CreateWindowAux::new()).unwrap();
        
        change_window_attributes(self.conn, frame_id, 
            &ChangeWindowAttributesAux::new()
            .event_mask(EventMask::SUBSTRUCTURE_NOTIFY | EventMask::SUBSTRUCTURE_REDIRECT)
            .border_pixel(0xff0000)).unwrap();
            
        reparent_window(self.conn, map_request.window, frame_id, 0, 0).unwrap();

        map_window(self.conn, frame_id).unwrap();
        map_window(self.conn, map_request.window).unwrap();

        self.conn.flush().unwrap();
    }
}