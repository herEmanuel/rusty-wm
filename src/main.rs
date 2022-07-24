use x11rb::{rust_connection::RustConnection, connection::Connection};

mod wm;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).expect("Could not connect to the x server");
    let screen = &conn.setup().roots[screen_num];

    let window_manager = wm::WindowManager::new(&conn, screen);
    window_manager.run();
}
