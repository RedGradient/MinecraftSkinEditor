use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ButtonExt, GtkWindowExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

mod imp;

glib::wrapper! {
    pub struct SkinDialog(ObjectSubclass<imp::SkinDialog>)
        @extends gtk::Widget, gtk::Window;
}

impl SkinDialog {
    pub fn new(texture_path: String) -> SkinDialog {
        let skin_dialog: SkinDialog = glib::Object::new();
        skin_dialog.imp().texture_path.borrow_mut().replace(texture_path);
        skin_dialog
    }
}