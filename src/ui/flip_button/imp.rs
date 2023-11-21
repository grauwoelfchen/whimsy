use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::Button;

#[derive(Default)]
pub struct FlipButton;

#[glib::object_subclass]
impl ObjectSubclass for FlipButton {
    const NAME: &'static str = "FlipButton";

    type Type = super::FlipButton;
    type ParentType = Button;
}

impl ObjectImpl for FlipButton {}

impl WidgetImpl for FlipButton {}

impl ButtonImpl for FlipButton {}
