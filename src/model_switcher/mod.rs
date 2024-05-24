use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::ToggleButtonExt;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct ModelSwitcher(ObjectSubclass<imp::ModelSwitcher>)
        @extends gtk::Widget;
}

impl ModelSwitcher {
    pub fn new() -> ModelSwitcher {
        Object::builder().build()
    }
    
    pub fn head(&self) -> gtk::ToggleButton {
        self.imp().head.get()
    }

    pub fn torso(&self) -> gtk::ToggleButton {
        self.imp().body.get()
    }

    pub fn right_arm(&self) -> gtk::ToggleButton {
        self.imp().right_arm.get()
    }

    pub fn left_arm(&self) -> gtk::ToggleButton {
        self.imp().left_arm.get()
    }

    pub fn right_leg(&self) -> gtk::ToggleButton {
        self.imp().right_leg.get()
    }

    pub fn left_leg(&self) -> gtk::ToggleButton {
        self.imp().left_leg.get()
    }

    pub fn inner_layer_toggle(&self) -> gtk::ToggleButton {
        self.imp().inner_layer_toggle.get()
    }

    pub fn outer_layer_toggle(&self) -> gtk::ToggleButton {
        self.imp().outer_layer_toggle.get()
    }
}