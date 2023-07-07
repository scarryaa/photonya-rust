use std::borrow::BorrowMut;
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

use gtk::gio::Settings;
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::traits::{ButtonExt, DialogExt, FileChooserExt, GtkWindowExt};
use gtk::{glib, Button, CompositeTemplate, FileChooserDialog, ResponseType};
use gtk::{Inhibit, Picture};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/nyan-inc/photonya/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub picture: TemplateChild<Picture>,
    #[template_child]
    pub file_chooser: TemplateChild<Button>,
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
        let weak_obj = self.downgrade();
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();

        let picture = self.picture.clone();

        self.file_chooser.connect_clicked(move |_| {
            let obj = weak_obj
                .upgrade()
                .expect("Window was dropped while button was pressed");

            let dialog = FileChooserDialog::new(
                Some("Open File"),
                None::<&gtk::Window>,
                gtk::FileChooserAction::Open,
                &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
            );

            let weak_obj_clone = weak_obj.clone();
            dialog.connect_response(move |dialog, response| {
                if response == Into::<i32>::into(ResponseType::Ok) {
                    let file_name = dialog.file().expect("Couldn't get filename");
                    println!("File selected: {:?}", file_name);

                    let obj: glib::subclass::ObjectImplRef<Window> = weak_obj_clone
                        .upgrade()
                        .expect("Window was dropped while dialog was open");
                    let picture = &obj.picture;
                    picture.set_file(Some(&file_name));
                }
                dialog.close();
            });

            dialog.present();
        });
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
