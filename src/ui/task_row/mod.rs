mod imp;

use glib::Object;
use gtk::glib;

use super::task_object::TaskObject;

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
    gtk::Orientable;
}

impl TaskRow {
    pub fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }

    pub fn bind(&self, _: &TaskObject) {
        // TODO
    }

    pub fn unbind(&self) {
        // TODO
    }
}

#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
