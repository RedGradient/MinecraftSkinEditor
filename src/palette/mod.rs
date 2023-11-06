use gtk::glib;
use gtk::glib::Object;

mod imp;

glib::wrapper! {
    pub struct Palette(ObjectSubclass<imp::Palette>)
        @extends gtk::Widget;
}

impl Palette {
    pub fn new() -> Self {
        Object::builder().build()
    }
}