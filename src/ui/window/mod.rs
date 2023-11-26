mod imp;

use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection};

use super::task_object::TaskObject;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
            gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("could not load current tasks")
    }

    fn setup_tasks(&self) {
        let model = gio::ListStore::new::<TaskObject>();

        // set the model as tasks
        self.imp().tasks.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.tasks()));
        self.imp().tasks_list.set_model(Some(&selection_model));
    }
}
