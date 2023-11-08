use gtk::glib;
use gtk::prelude::GtkApplicationExt;
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

impl ObjectImpl for Application {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_accels_for_action("win.undo", &["<Primary>Z", "<Meta>Z"]);
        obj.set_accels_for_action("win.redo", &["<Primary><Shift>Z", "<Meta><Shift>Z"]);
    }
}
impl ApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
impl AdwApplicationImpl for Application {}
