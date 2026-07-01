use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ToggleButtonExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::window::Window;

pub(super) fn connect(win: &Window) {
    win.imp().wardrobe.connect_toggled(clone!(#[weak(rename_to = win)] win, move |btn| {
        win.imp().left_box.set_visible(!btn.is_active());
        win.imp().right_box.set_visible(!btn.is_active());
        win.imp().template_list.set_visible(btn.is_active());

        if btn.is_active() {
            win.imp().template_list.load_list(&win.clone());
        }

        win.set_tool_active(!btn.is_active());
    }));
}
