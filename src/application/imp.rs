use gtk::glib;
use libadwaita as adw;
use libadwaita::subclass::prelude::*;

#[derive(Default)]
pub struct Application;

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "MCSkinEditorApplication";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {}
impl ApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
impl AdwApplicationImpl for Application {}
