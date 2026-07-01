use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::ToggleButtonExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::command::Tool;
use crate::window::Window;

pub(super) fn connect(win: &Window) {
    win.imp().pencil.connect_toggled(clone!(#[weak(rename_to = win)] win, move |_| {
        win.editor_mut().set_tool(Tool::Pencil);
    }));
    win.imp().rubber.connect_toggled(clone!(#[weak(rename_to = win)] win, move |_| {
        win.editor_mut().set_tool(Tool::Rubber);
    }));
    win.imp().color_picker.connect_toggled(clone!(#[weak(rename_to = win)] win, move |_| {
        win.editor_mut().set_tool(Tool::ColorPicker);
    }));
    win.imp().fill.connect_toggled(clone!(#[weak(rename_to = win)] win, move |_| {
        win.editor_mut().set_tool(Tool::Fill);
    }));
    win.imp().random_color.connect_toggled(clone!(#[weak(rename_to = win)] win, move |_| {
        win.editor_mut().set_tool(Tool::Random);
    }));
    win.imp().replace_color.connect_toggled(clone!(#[weak(rename_to = win)] win, move |_| {
        win.editor_mut().set_tool(Tool::Replace);
    }));
}
