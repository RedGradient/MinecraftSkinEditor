use gtk::{gio, glib};
use gtk::glib::Object;
use libadwaita as adw;

use crate::APP_ID;

mod imp;

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", APP_ID)
            .build()
    }
}