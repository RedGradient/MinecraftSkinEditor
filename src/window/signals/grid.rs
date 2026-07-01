use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::ToggleButtonExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::window::Window;

pub(super) fn connect(win: &Window) {
    win.imp().grid_toggle.connect_toggled(clone!(#[weak(rename_to = win)] win, move |btn| {
        let renderer = win.gl_area().renderer().unwrap();
        let mut renderer = renderer.borrow_mut();
        renderer.set_grid_show(btn.is_active());
        win.request_viewport_redraw();
    }));
}
