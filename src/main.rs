mod arcane_app;
mod render_mender;

use arcane_app::ArcaneApp;
use render_mender::sdl_app::SdlApp;
use std::ffi::CString;

fn main() {
    let title = CString::new("Blit").unwrap();
    let mut s_app = SdlApp::new(&title);
    let mut arcane_app = ArcaneApp::default();

    s_app.main_loop(&mut arcane_app);
}
