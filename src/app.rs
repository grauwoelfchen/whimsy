mod ui;

use gtk::prelude::*;
use gtk::{gio, glib::ExitCode, Application};

use ui::window::Window;

static APP_ID: &str = "app.yasha.whimsy";

fn main() -> ExitCode {
    gio::resources_register_include!("whimsy.gresource")
        .expect("Failed to register resource file.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let win = Window::new(app);
    win.present();
}
