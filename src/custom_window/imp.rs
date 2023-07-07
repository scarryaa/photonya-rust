use std::cell::OnceCell;

use gtk::gio::Settings;
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::traits::ButtonExt;
use gtk::{glib, Button, CompositeTemplate};
use gtk::{Inhibit, Picture};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/nyan-inc/photonya/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub picture: TemplateChild<Picture>,
    #[template_child]
    pub fileChooser: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "photonyaWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    // Save window state before the window is closed
    fn close_request(&self) -> glib::signal::Inhibit {
        self.obj()
            .save_window_size()
            .expect("Failed to save window size");

        Inhibit(false)
    }
}

impl ApplicationWindowImpl for Window {}
