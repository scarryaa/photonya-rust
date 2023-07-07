mod custom_window;

use custom_window::Window;
use gtk::{
    gio::{self},
    prelude::*,
    traits::GtkWindowExt,
    Application,
};

const APP_ID: &str = "org.nyan-inc.photonya.WindowSettings";
// const IMAGE_DATA: &[u8] = include_bytes!("assets/image.png");

fn main() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    let app: Application = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
