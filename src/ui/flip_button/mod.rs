mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct FlipButton(ObjectSubclass<imp::FlipButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl FlipButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for FlipButton {
    fn default() -> Self {
        Self::new()
    }
}
