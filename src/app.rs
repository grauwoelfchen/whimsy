mod ui;

use std::env;

use gtk::prelude::*;
use gtk::{gio, glib::ExitCode, Application, Settings};

use ui::window::Window;

static APP_ID: &str = "app.yasha.whimsy";

fn main() -> ExitCode {
    gtk::init().expect("failed to initialize GTK.");

    gio::resources_register_include!("whimsy.gresource")
        .expect("Failed to register resource file.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let win = Window::new(app);

    let settings = Settings::default().unwrap();
    if let Ok(im_module) = env::var("GTK_IM_MODULE") {
        settings.set_gtk_im_module(Some(&im_module));
        // FIXME:
        //
        // This is a temporary (and bad) solution for a bit annoying warning
        // message about missing IM modules as below:
        // Gtk-WARNING **: <...>: No IM module matching GTK_IM_MODULE=uim found
        //
        // It seems that some IMs still don't work on
        // GTK4 applications yet (eg. uim). This is because of changes in GTK4
        // as explained at here:
        // https://blog.gtk.org/2018/03/06/input-methods-in-gtk-4/
        //
        // Let's take a look at it later.
        env::remove_var("GTK_IM_MODULE");
    }

    win.present();
}
