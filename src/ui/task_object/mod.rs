mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
    gtk::Orientable;
}

impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
}

#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
