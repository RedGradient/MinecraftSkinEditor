use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ButtonExt, ToggleButtonExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::window::Window;

pub(super) fn connect(win: &Window) {
    win.imp().reset_skin_button.connect_clicked(clone!(#[weak(rename_to = win)] win, move |_| {
        win.reset_skin();
    }));
}
