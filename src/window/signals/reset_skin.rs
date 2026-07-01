use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ButtonExt, ToggleButtonExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::glium_area::renderer::Renderer;
use crate::window::Window;

pub(super) fn connect(win: &Window) {
    win.imp().reset_skin_button.connect_clicked(clone!(#[weak(rename_to = win)] win, move |_| {
        let renderer = win.gl_area().renderer().unwrap();
        let mut renderer: std::cell::RefMut<Renderer> = renderer.borrow_mut();
        renderer.reset_skin();
        drop(renderer);
        win.imp().grid_toggle.set_active(true);
        win.request_viewport_redraw();
    }));
}
