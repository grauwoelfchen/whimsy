use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, ApplicationWindow, CompositeTemplate, Label, TemplateChild};
use rand::Rng;

use crate::ui::flip_button::FlipButton;

#[derive(CompositeTemplate, Default)]
#[template(file = "../window.ui")]
pub struct Window {
    #[template_child(id = "label")]
    pub label: TemplateChild<Label>,

    #[template_child(id = "button")]
    pub button: TemplateChild<FlipButton>,
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(&self, btn: &FlipButton) {
        let lbl = self.label.clone();

        #[rustfmt::skip]
        glib::spawn_future_local(
            glib::clone!(@strong btn, @strong lbl => async move {
                let txt = btn.label().unwrap_or(glib::GString::from(""));
                btn.set_sensitive(false);
                btn.set_label("Flipping");

                flip_coin(&lbl).await;

                btn.set_label(&txt);
                btn.set_sensitive(true);
            }),
        );
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "AppWindow";

    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        FlipButton::ensure_type();

        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

/// Simulate coin flipping action.
///
/// This function randomly sleeps a bit and changes the label text.
/// Do not call this in the main theard.
async fn flip_coin(lbl: &Label) {
    lbl.set_text("-");

    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=5);

    glib::timeout_future_seconds(n).await;

    // TODO: pick up a random task
    if rand::random() {
        lbl.set_text("Heads");
    } else {
        lbl.set_text("Tails");
    }
}
