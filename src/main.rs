mod render_mender;

use render_mender::sdl_app::SdlApp;
use std::ffi::CString;

fn main() {
    let title = CString::new("Blit").unwrap();
    let mut s_app = SdlApp::new(&title);

    s_app.main_loop();
}
